
mod navtab;
mod content;
mod ops;

use termion::event::Key;
use tui::layout::{ Constraint, Direction, Layout, Rect };

use crate::scene::navtab::NavTabPainter;
use crate::scene::content::ContentPainter;
use crate::scene::ops::OperationPainter;
use crate::config::tab::TabsConfig;
use crate::config::setting::SettingConfig;
use crate::config::manifest::EXIT_KEY;
use crate::config::ConfigOp;
use crate::utils::{ THLEvents, THLEvent };


pub trait TerminalPainter {

    fn draw(&mut self, f: &mut crate::DstFrame, area: Rect);
}

pub struct THLScene {

    layout: Layout,

    navtab  : NavTabPainter,
    content : ContentPainter,
    ops     : OperationPainter,
}

impl THLScene {

    pub fn new(tabs: TabsConfig) -> THLScene {

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(8),
            ].as_ref());

        THLScene {
            layout : chunks,
            navtab : NavTabPainter::new(&tabs),
            content: ContentPainter::new(tabs),
            ops    : OperationPainter::new(),
        }
    }

    pub fn react(&mut self, reaction: SceneReaction) -> ConfigOp {

        match reaction {
            | SceneReaction::LaunchGame => {

                if let Some(current_program) = self.content.current_program() {

                    self.ops.switch_mode(THLOperation::Running, Some(current_program.name.clone()));
                    let running_result = self.content.launch();
                    self.ops.switch_mode(THLOperation::Common, None);

                    running_result.and_then(|running_status| {
                        use std::error::Error;

                        match running_status {
                            | Ok(status) => {
                                if status.success() == false {
                                    Some(format!("error code: {:?}", status.code()))
                                } else {
                                    None
                                }
                            },
                            | Err(e) => Some(String::from(e.description())),
                        }
                    }).and_then(|hint| {
                        Some(self.ops.set_running_error_hint(&hint))
                    });
                }
            },
            | SceneReaction::NextTab => {
                self.navtab.state.next();

                let current_tab = self.navtab.current_index();
                self.content.set_tab(current_tab);
                self.ops.set_tab(current_tab)
            },
            | SceneReaction::PreviousTab => {
                self.navtab.state.previous();

                let current_tab = self.navtab.current_index();
                self.content.set_tab(current_tab);
                self.ops.set_tab(current_tab);
            },
            | SceneReaction::NextGame => self.content.next_tab(),
            | SceneReaction::PreviousGame => self.content.previous_tab(),
            | SceneReaction::CancelOp => self.ops.cancel_op(),
            | SceneReaction::ConfirmAction => {
                let ops = self.ops.confirm_op();
                self.update_config(&ops);
                return ops
            },
            | SceneReaction::AppendTab => self.ops.switch_mode(THLOperation::AppendingTab, None),
            | SceneReaction::RemoveTab => self.ops.switch_mode(THLOperation::RemovingTab, None),
            | SceneReaction::AppendGame => self.ops.switch_mode(THLOperation::AppendingGame, None),
            | SceneReaction::RemoveGame => self.ops.switch_mode(THLOperation::RemovingGame, None),
            | SceneReaction::UserInput(key) => self.ops.input_word(key),
            | SceneReaction::SwitchInputFocus => self.ops.swtich_input_focus(),
        }

        ConfigOp::None
    }

    pub fn draw(&mut self, f: &mut crate::DstFrame) {

        let chunks = self.layout.clone()
            .split(f.size());

        self.navtab.draw(f, chunks[0]);
        self.content.draw(f, chunks[1]);
        self.ops.draw(f, chunks[2]);
    }

    fn update_config(&mut self, ops: &ConfigOp) {
        self.navtab.update_tabs(ops);
        self.content.update_tab(ops);
    }
}

#[derive(Debug)]
pub enum SceneAction {
    Terminal,
    Rendering,
    React(SceneReaction),
}

#[derive(Debug)]
pub enum SceneReaction {
    NextTab,    PreviousTab,
    NextGame,   PreviousGame, LaunchGame,
    AppendTab,  RemoveTab,
    AppendGame, RemoveGame, SwitchInputFocus,
    CancelOp,   ConfirmAction,
    UserInput(Key),
}

pub struct EventNerve {

    event_loop: THLEvents,

    op: THLOperation,
}

#[derive(Debug)]
pub enum THLOperation {
    Common,
    Running,
    AppendingGame,
    RemovingGame,
    AppendingTab,
    RemovingTab,
}

impl EventNerve {

    pub fn new(config: SettingConfig) -> EventNerve {

        EventNerve {
            event_loop: THLEvents::with_config(config),
            op: THLOperation::Common,
        }
    }

    pub fn tick(&mut self) -> Result<SceneAction, failure::Error> {

        if let THLEvent::Input(key) = self.event_loop.next()? {

            match self.op {
                | THLOperation::Common => {

                    match key {
                        | EXIT_KEY   => return Ok(SceneAction::Terminal),
                        | Key::Right => return Ok(SceneAction::React(SceneReaction::NextTab)),
                        | Key::Left  => return Ok(SceneAction::React(SceneReaction::PreviousTab)),
                        | Key::Down  => return Ok(SceneAction::React(SceneReaction::NextGame)),
                        | Key::Up    => return Ok(SceneAction::React(SceneReaction::PreviousGame)),
                        | Key::Char('\n') => {
                            return Ok(SceneAction::React(SceneReaction::LaunchGame))
                        },
                        | Key::Ctrl('n') => {
                            self.op = THLOperation::AppendingGame;
                            return Ok(SceneAction::React(SceneReaction::AppendGame))
                        },
                        | Key::Ctrl('d') => {
                            self.op = THLOperation::RemovingGame;
                            return Ok(SceneAction::React(SceneReaction::RemoveGame))
                        },
                        | Key::Ctrl('t') => {
                            self.op = THLOperation::AppendingTab;
                            return Ok(SceneAction::React(SceneReaction::AppendTab))
                        },
                        | Key::Ctrl('r') => {
                            self.op = THLOperation::RemovingTab;
                            return Ok(SceneAction::React(SceneReaction::RemoveTab))
                        },
                        | _ => {},
                    }
                },
                | THLOperation::AppendingGame => {
                    match key {
                        | Key::Esc => {
                            self.op = THLOperation::Common;
                            return Ok(SceneAction::React(SceneReaction::CancelOp))
                        },
                        | Key::Char('\n') => {
                            self.op = THLOperation::Common;
                            return Ok(SceneAction::React(SceneReaction::ConfirmAction))
                        },
                        | Key::Char(_)
                        | Key::Delete
                        | Key::Backspace => return Ok(SceneAction::React(SceneReaction::UserInput(key))),
                        | Key::Up
                        | Key::Down => return Ok(SceneAction::React(SceneReaction::SwitchInputFocus)),
                        | _ => {},
                    }
                },
                | THLOperation::AppendingTab
                | THLOperation::RemovingGame
                | THLOperation::RemovingTab => {
                    match key {
                        | Key::Esc => {
                            self.op = THLOperation::Common;
                            return Ok(SceneAction::React(SceneReaction::CancelOp))
                        },
                        | Key::Char('\n') => {
                            self.op = THLOperation::Common;
                            return Ok(SceneAction::React(SceneReaction::ConfirmAction))
                        },
                        | Key::Char(_)
                        | Key::Delete
                        | Key::Backspace => return Ok(SceneAction::React(SceneReaction::UserInput(key))),
                        | _ => {},
                    }
                },
                | THLOperation::Running => {
                    unreachable!()
                },
            }
        }

        Ok(SceneAction::Rendering)
    }
}
