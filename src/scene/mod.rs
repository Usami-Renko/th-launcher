
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
use crate::config::{ ConfigOp, ConfigError };
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
                Constraint::Percentage(15),
                Constraint::Percentage(50),
                Constraint::Percentage(40),
            ].as_ref());

        THLScene {
            layout : chunks,
            navtab : NavTabPainter::new(&tabs),
            content: ContentPainter::new(tabs),
            ops    : OperationPainter::new(),
        }
    }

    pub fn react(&mut self, reaction: SceneReaction) -> Result<ConfigOp, ConfigError> {

        match reaction {
            | SceneReaction::LaunchGame => self.content.launch(),
            | SceneReaction::NextTab => {
                let current_tab = self.navtab.current_index();

                self.navtab.state.next();
                self.content.set_tab(current_tab);
                self.ops.set_tab(current_tab)
            },
            | SceneReaction::PreviousTab => {
                let current_tab = self.navtab.current_index();

                self.navtab.state.previous();
                self.content.set_tab(current_tab);
                self.ops.set_tab(current_tab);
            },
            | SceneReaction::CancelOp => self.ops.cancel_op(),
            | SceneReaction::ConfirmAction =>{
                let ops = self.ops.confirm_op()?;
                self.update_config(&ops);
                return Ok(ops)
            },
            | SceneReaction::NextGame => self.content.state.next(),
            | SceneReaction::PreviousGame => self.content.state.previous(),
            | SceneReaction::AppendTab => self.ops.switch_mode(THLOperation::AppendingTab),
            | SceneReaction::RemoveTab => self.ops.switch_mode(THLOperation::RemovingTab),
            | SceneReaction::AppendGame => self.ops.switch_mode(THLOperation::AppendingGame),
            | SceneReaction::RemoveGame => self.ops.switch_mode(THLOperation::RemovingGame),
            | SceneReaction::UserInput(key) => self.ops.input_word(key),
            | SceneReaction::SwitchInputFocus => self.ops.swtich_input_focus(),
        }

        Ok(ConfigOp::None)
    }

    pub fn draw(&mut self, f: &mut crate::DstFrame) {

        let chunks = self.layout.clone()
            .split(f.size());

        self.navtab.draw(f, chunks[0]);

        self.content.set_tab(self.navtab.current_index());
        self.content.draw(f, chunks[1]);

        self.ops.draw(f, chunks[2]);
    }

    fn update_config(&mut self, ops: &ConfigOp) {
        self.navtab.update_tabs(ops);
        self.content.update_tab(ops);
    }
}

pub enum SceneAction {
    Terminal,
    Rendering,
    React(SceneReaction),
}

pub enum SceneReaction {
    NextTab,    PreviousTab,
    NextGame,   PreviousGame, LaunchGame,
    AppendTab,  RemoveTab,
    AppendGame, RemoveGame, SwitchInputFocus,
    CancelOp,   ConfirmAction,
    UserInput(Key),
}

pub struct EventNerve {

    pub is_active: bool,
    event_loop: THLEvents,

    op: THLOperation,
}

#[derive(Debug)]
pub enum THLOperation {
    Common,
    AppendingGame,
    RemovingGame,
    AppendingTab,
    RemovingTab,
}

impl EventNerve {

    pub fn new(config: SettingConfig) -> EventNerve {

        EventNerve {
            is_active: false,
            event_loop: THLEvents::with_config(config),
            op: THLOperation::Common,
        }
    }

    pub fn tick(&mut self) -> Result<SceneAction, failure::Error> {

        match self.event_loop.next()? {
            | THLEvent::Input(key) => {
                self.is_active = true;

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
                    | THLOperation::AppendingTab => {
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
                            | _ => {},
                        }
                    },
                }
            },
            | _ => {
                self.is_active = false;
            },
        }

        Ok(SceneAction::Rendering)
    }
}
