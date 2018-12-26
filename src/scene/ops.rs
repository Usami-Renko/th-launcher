
use termion::event::Key;
use tui::layout::{ Layout, Rect, Alignment, Direction, Constraint };
use tui::style::{ Style, Color };
use tui::widgets::{ Block, Text, Paragraph, Borders, Widget };

use std::path::Path;

use crate::scene::TerminalPainter;
use crate::scene::THLOperation;
use crate::config::{ ConfigOp, ConfigError };
use crate::config::tab::ItemConfig;

pub struct OperationPainter {

    block: Block<'static>,
    layout: Layout,

    current_tab: usize,
    instruction: InstructionType,
}

impl TerminalPainter for OperationPainter {

    fn draw(&mut self, f: &mut crate::DstFrame, area: Rect) {

        let chunks = self.layout.clone().split(area);

        self.block.render(f, area);

        match self.instruction {
            | InstructionType::Common(ref v) => {
                v.draw_ops(f, chunks[0]);
                v.draw_hints(f, chunks[1]);
            },
            | InstructionType::NewGame(ref v) => {
                v.draw_ops(f, chunks[0]);
                v.draw_hints(f, chunks[1]);
            }
        }
    }
}

impl OperationPainter {

    pub fn new() -> OperationPainter {

        let block = Block::default()
            .title("Instruction")
            .borders(Borders::ALL);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Max(2)].as_ref());

        OperationPainter {
            block, layout,
            current_tab: 0,
            instruction: InstructionType::Common(CommonInstruction),
        }
    }

    pub fn set_tab(&mut self, index: usize) {
        self.current_tab = index;
    }

    pub fn switch_mode(&mut self, op: THLOperation) {
        match op {
            | THLOperation::Common => self.instruction = InstructionType::Common(CommonInstruction),
            | THLOperation::AppendingGame => self.instruction = InstructionType::NewGame(NewGameInstruction::new()),
            | _ => {
                unimplemented!()
            }
        }
    }

    pub fn input_word(&mut self, key: Key) {

        if let InstructionType::NewGame(ref mut inst) = self.instruction {
            inst.receive_input(key);
        }
    }

    pub fn swtich_input_focus(&mut self) {

        if let InstructionType::NewGame(ref mut inst) = self.instruction {
            inst.switch_focus();
        }
    }

    pub fn cancel_op(&mut self) {

        match self.instruction {
            | InstructionType::NewGame(_) => self.instruction = InstructionType::Common(CommonInstruction),
            | _ => {},
        }
    }

    pub fn confirm_op(&mut self) -> Result<ConfigOp, ConfigError> {

        match self.instruction {
            | InstructionType::NewGame(ref inst) => {

                if inst.input_name.is_empty() {
                    return Err(ConfigError::NameEmpty)
                }
                if inst.input_path.is_empty() {
                    return Err(ConfigError::PathEmpty)
                }
                let path = Path::new(&inst.input_path);
                if path.is_file() == false {
                    return Err(ConfigError::PathInvalid)
                }

                let result = ConfigOp::AppendGame {
                    tab_index: self.current_tab,
                    config: ItemConfig {
                        name: inst.input_name.clone(),
                        path: inst.input_path.clone(),
                    }
                };
                Ok(result)
            },
            | _ => Ok(ConfigOp::None),
        }
    }
}

trait DrawableInstruction where Self: Sized {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect);
    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect);
}

enum InstructionType {
    Common(CommonInstruction),
    NewGame(NewGameInstruction),
}

pub struct CommonInstruction;

impl DrawableInstruction for CommonInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("[Ctrl + n]Append a new game.\n"),
            Text::raw("[Ctrl + d]Remove a game.\n"),
            Text::raw("[Ctrl + t]Append a new tab.\n"),
            Text::raw("[Ctrl + r]Remove a tab.\n"),
        ];

        Paragraph::new(texts.iter())
            .alignment(Alignment::Left)
            .render(f, area);
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Use arrow key to select game.\n"),
            Text::raw("Press ESC to quit the program."),
        ];

        Paragraph::new(texts.iter())
            .alignment(Alignment::Left)
            .render(f, area);
    }
}

pub struct NewGameInstruction {

    layout: Layout,

    focus: InputFocus,
    input_name: String,
    input_path: String,

    text_style: Style,
}

enum InputFocus { Name, Path }

impl DrawableInstruction for NewGameInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let chunks = self.layout.clone().split(area);

        let input_texts = [
            Text::raw("Name: "),
            Text::raw(&self.input_name),
        ];
        let path_texts = [
            Text::raw("Path: "),
            Text::raw(&self.input_path),
        ];

        Paragraph::new(input_texts.iter())
            .style(self.text_style)
            .render(f, chunks[0]);
        Paragraph::new(path_texts.iter())
            .style(self.text_style)
            .render(f, chunks[1]);
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Press Enter to confirm.\n"),
            Text::raw("Press ESC to cancel.\n"),
        ];

        Paragraph::new(texts.iter())
            .alignment(Alignment::Left)
            .render(f, area);
    }
}

impl NewGameInstruction {

    pub fn new() -> NewGameInstruction {

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref());

        NewGameInstruction {
            layout,
            focus: InputFocus::Name,
            input_name: String::new(),
            input_path: String::new(),
            text_style: Style::default().fg(Color::Yellow),
        }
    }

    fn switch_focus(&mut self) {
        match self.focus {
            | InputFocus::Name => self.focus = InputFocus::Path,
            | InputFocus::Path => self.focus = InputFocus::Name,
        }
    }

    fn receive_input(&mut self, key: Key) {

        match key {
            | Key::Backspace => {
                match self.focus {
                    | InputFocus::Name => { self.input_name.pop(); },
                    | InputFocus::Path => { self.input_path.pop(); },
                }
            },
            | Key::Char(ch) => {
                match self.focus {
                    | InputFocus::Name => self.input_name.push(ch),
                    | InputFocus::Path => self.input_path.push(ch),
                }
            },
            | _ => {},
        }
    }
}
