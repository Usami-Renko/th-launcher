
use termion::event::Key;
use tui::layout::{ Layout, Rect, Direction, Constraint };
use tui::style::{ Style, Color };
use tui::widgets::{ Block, Text, Paragraph, Borders, Widget };

use std::path::Path;
use std::str::FromStr;

use crate::scene::TerminalPainter;
use crate::scene::THLOperation;
use crate::config::ConfigOp;
use crate::config::tab::{ TabConfig, ItemConfig };

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
            | InstructionType::Running(ref v) => {
                v.draw_ops(f, chunks[0]);
            },
            | InstructionType::NewGame(ref v) => {
                v.draw_ops(f, chunks[0]);
                v.draw_hints(f, chunks[1]);
            },
            | InstructionType::NewTab(ref v) => {
                v.draw_ops(f, chunks[0]);
                v.draw_hints(f, chunks[1]);
            },
            | InstructionType::RemoveGame(ref v) => {
                v.draw_ops(f, chunks[0]);
                v.draw_hints(f, chunks[1]);
            },
            | InstructionType::RemoveTab(ref v) => {
                v.draw_ops(f, chunks[0]);
                v.draw_hints(f, chunks[1]);
            },
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
            .constraints([Constraint::Min(3), Constraint::Length(2)].as_ref());

        OperationPainter {
            block, layout,
            current_tab: 0,
            instruction: InstructionType::Common(CommonInstruction::new()),
        }
    }

    pub fn set_tab(&mut self, index: usize) {
        self.current_tab = index;
    }

    pub fn switch_mode(&mut self, op: THLOperation, mess: Option<String>) {
        match op {
            | THLOperation::Common        => self.instruction = InstructionType::Common(CommonInstruction::new()),
            | THLOperation::AppendingGame => self.instruction = InstructionType::NewGame(NewGameInstruction::new()),
            | THLOperation::AppendingTab  => self.instruction = InstructionType::NewTab(NewTabInstruction::new()),
            | THLOperation::RemovingGame  => self.instruction = InstructionType::RemoveGame(RemoveGameInstruction::new()),
            | THLOperation::RemovingTab   => self.instruction = InstructionType::RemoveTab(RemoveTabInstruction::new()),
            | THLOperation::Running       => self.instruction = InstructionType::Running(RunningInstruction::new(mess.unwrap())),
        }
    }

    pub fn input_word(&mut self, key: Key) {

        match self.instruction {
            | InstructionType::NewGame(ref mut inst)    => inst.receive_input(key),
            | InstructionType::NewTab(ref mut inst)     => inst.receive_input(key),
            | InstructionType::RemoveGame(ref mut inst) => inst.receive_input(key),
            | InstructionType::RemoveTab(ref mut inst)  => inst.receive_input(key),
            | _ => unreachable!(),
        }
    }

    pub fn swtich_input_focus(&mut self) {

        if let InstructionType::NewGame(ref mut inst) = self.instruction {
            inst.switch_focus();
        }
    }

    pub fn cancel_op(&mut self) {

        self.instruction = InstructionType::Common(CommonInstruction::new());
    }

    pub fn set_running_error_hint(&mut self, mess: &str) {

        if let InstructionType::Common(ref mut inst) = self.instruction {
            inst.hint = Some(String::from(format!("Some errors occur during the program running: {}", mess)));
        }
    }

    pub fn confirm_op(&mut self) -> ConfigOp {

        let (result, instruction) = match self.instruction {
            | InstructionType::NewGame(ref inst) => {

                let mut new_inst = CommonInstruction::new();

                let path = Path::new(&inst.input_path);

                let mut is_success = true;
                if inst.input_name.is_empty() {
                    is_success = false;
                    new_inst.hint = Some(String::from("Operation failed. Name must not be empty."));
                }
                if inst.input_path.is_empty() {
                    is_success = false;
                    new_inst.hint = Some(String::from("Operation failed. Path must not be empty."));
                }
                if path.is_file() == false {
                    is_success = false;
                    new_inst.hint = Some(String::from("Operation failed. Path is not an valid value."));
                }

                let result = if is_success {
                    ConfigOp::AppendGame {
                        tab_index: self.current_tab,
                        config: ItemConfig {
                            name: inst.input_name.clone(),
                            path: inst.input_path.clone(),
                        }
                    }
                } else {
                    ConfigOp::None
                };

                (result, InstructionType::Common(new_inst))
            },
            | InstructionType::NewTab(ref inst) => {

                let mut new_inst = CommonInstruction::new();

                let result = if inst.input_name.is_empty() {
                    new_inst.hint = Some(String::from("Operation failed. Name must not be empty."));
                    ConfigOp::None
                } else {
                    ConfigOp::AppendTab {
                        config: TabConfig {
                            name: inst.input_name.clone(),
                            items: vec![],
                        }
                    }
                };

                (result, InstructionType::Common(new_inst))
            },
            | InstructionType::RemoveGame(ref mut inst) => {

                let mut new_inst = CommonInstruction::new();

                let result = match usize::from_str(&inst.input_content) {
                    | Ok(game_index) => {
                        ConfigOp::RemoveGame {
                            tab_index: self.current_tab,
                            item_index: game_index,
                        }
                    },
                    | Err(_) => {
                        new_inst.hint = Some(String::from("Operation failed. Input content is not a valid integer."));
                        ConfigOp::None
                    }
                };

                (result, InstructionType::Common(new_inst))
            },
            | InstructionType::RemoveTab(ref mut inst) => {

                let mut new_inst = CommonInstruction::new();

                let result = match usize::from_str(&inst.input_content) {
                    | Ok(tab_index) => {
                        ConfigOp::RemoveTab {
                            tab_index,
                        }
                    },
                    | Err(_) => {
                        new_inst.hint = Some(String::from("Operation failed. Input content is not a valid integer."));
                        ConfigOp::None
                    }
                };

                (result, InstructionType::Common(new_inst))
            },
            | InstructionType::Common(ref mut inst) => {

                inst.hint = None;
                return ConfigOp::None
            },
            | InstructionType::Running(_) => {
                unreachable!()
            }
        };

        self.instruction = instruction;
        result
    }
}

trait DrawableInstruction where Self: Sized {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect);
    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect);
}

enum InstructionType {

    Common(CommonInstruction),
    Running(RunningInstruction),
    NewGame(NewGameInstruction),
    NewTab(NewTabInstruction),
    RemoveGame(RemoveGameInstruction),
    RemoveTab(RemoveTabInstruction),
}


// Instruction. -------------------------------------------------------------------------
struct CommonInstruction {

    hint: Option<String>,
    style_hint: Style,
    ops_layout: Layout,
}

impl DrawableInstruction for CommonInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("[Ctrl + n]Append a new game.  "),
            Text::raw("[Ctrl + t]Append a new tab.\n"),
            Text::raw("[Ctrl + d]Remove a game.      "),
            Text::raw("[Ctrl + r]Remove a tab.\n"),
        ];

        if let Some(ref hint) = self.hint {

            let chunks = self.ops_layout.clone().split(area);
            Paragraph::new(texts.iter())
                .render(f, chunks[0]);
            Paragraph::new([Text::raw(hint)].iter()).style(self.style_hint)
                .render(f, chunks[1]);
        } else {
            Paragraph::new(texts.iter())
                .render(f, area);
        }
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Use arrow key to select game and tab.\n"),
            Text::raw("Press ESC to quit the program."),
        ];

        Paragraph::new(texts.iter())
            .render(f, area);
    }
}

impl CommonInstruction {

    fn new() -> CommonInstruction {
        CommonInstruction {
            hint: None,
            style_hint: Style::default().fg(Color::Red),
            ops_layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Length(1)].as_ref()),
        }
    }
}
// --------------------------------------------------------------------------------------

// Instruction. -------------------------------------------------------------------------
struct NewGameInstruction {

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

        match self.focus {
            | InputFocus::Name => {
                let input_texts = [Text::raw("Game Name: "), Text::raw(&self.input_name), Text::raw("_")];
                Paragraph::new(input_texts.iter()).style(self.text_style)
                    .render(f, chunks[0]);

                let path_texts = [Text::raw("Game Path: "), Text::raw(&self.input_path)];
                Paragraph::new(path_texts.iter()).style(self.text_style)
                    .render(f, chunks[1]);
            },
            | InputFocus::Path => {
                let input_texts = [Text::raw("Game Name: "), Text::raw(&self.input_name)];
                Paragraph::new(input_texts.iter()).style(self.text_style)
                    .render(f, chunks[0]);

                let path_texts = [Text::raw("Game Path: "), Text::raw(&self.input_path), Text::raw("_")];
                Paragraph::new(path_texts.iter()).style(self.text_style)
                    .render(f, chunks[1]);
            }
        }
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Press Enter to confirm. Up and Down arrow to switch input filed.\n"),
            Text::raw("Press ESC to cancel."),
        ];

        Paragraph::new(texts.iter())
            .render(f, area);
    }
}

impl NewGameInstruction {

    pub fn new() -> NewGameInstruction {

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref());

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
// --------------------------------------------------------------------------------------


// Instruction. -------------------------------------------------------------------------
struct NewTabInstruction {

    input_name: String,
    text_style: Style,
}

impl DrawableInstruction for NewTabInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let input_texts = [
            Text::raw("Tab Name: "),
            Text::raw(&self.input_name),
        ];

        Paragraph::new(input_texts.iter())
            .style(self.text_style)
            .render(f, area);
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Press Enter to confirm.\n"),
            Text::raw("Press ESC to cancel."),
        ];

        Paragraph::new(texts.iter())
            .render(f, area);
    }
}

impl NewTabInstruction {

    fn new() -> NewTabInstruction {

        NewTabInstruction {
            input_name: String::new(),
            text_style: Style::default().fg(Color::Yellow),
        }
    }

    fn receive_input(&mut self, key: Key) {

        match key {
            | Key::Backspace => { self.input_name.pop(); }
            | Key::Char(ch)  => self.input_name.push(ch),
            | _ => {},
        }
    }
}
// --------------------------------------------------------------------------------------

// Instruction. -------------------------------------------------------------------------
struct RemoveGameInstruction {

    input_content: String,
}

impl DrawableInstruction for RemoveGameInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let input_texts = [
            Text::raw("Please input game index: "),
            Text::raw(&self.input_content),
        ];

        Paragraph::new(input_texts.iter())
            .render(f, area);
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Press Enter to confirm.\n"),
            Text::raw("Press ESC to cancel."),
        ];

        Paragraph::new(texts.iter())
            .render(f, area);
    }
}

impl RemoveGameInstruction {

    fn new() -> RemoveGameInstruction {

        RemoveGameInstruction {
            input_content: String::new(),
        }
    }

    fn receive_input(&mut self, key: Key) {

        match key {
            | Key::Backspace => { self.input_content.pop(); }
            | Key::Char(ch)  => self.input_content.push(ch),
            | _ => {},
        }
    }
}
// --------------------------------------------------------------------------------------

// Instruction. -------------------------------------------------------------------------
struct RemoveTabInstruction {

    input_content: String,
}

impl DrawableInstruction for RemoveTabInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let input_texts = [
            Text::raw("Please input tab index: "),
            Text::raw(&self.input_content),
        ];

        Paragraph::new(input_texts.iter())
            .render(f, area);
    }

    fn draw_hints(&self, f: &mut crate::DstFrame, area: Rect) {

        let texts = [
            Text::raw("Press Enter to confirm.\n"),
            Text::raw("Press ESC to cancel."),
        ];

        Paragraph::new(texts.iter())
            .render(f, area);
    }
}

impl RemoveTabInstruction {

    fn new() -> RemoveTabInstruction {

        RemoveTabInstruction {
            input_content: String::new(),
        }
    }

    fn receive_input(&mut self, key: Key) {

        match key {
            | Key::Backspace => { self.input_content.pop(); },
            | Key::Char(ch) => self.input_content.push(ch),
            | _ => {},
        }
    }
}
// --------------------------------------------------------------------------------------

// Instruction. -------------------------------------------------------------------------
struct RunningInstruction {

    program: String,
}

impl DrawableInstruction for RunningInstruction {

    fn draw_ops(&self, f: &mut crate::DstFrame, area: Rect) {

        let input_texts = [
            Text::raw("Running: "),
            Text::raw(&self.program),
            Text::raw("..."),
        ];

        Paragraph::new(input_texts.iter())
            .render(f, area);
    }

    fn draw_hints(&self, _: &mut crate::DstFrame, _: Rect) {
        // ignore this func...
    }
}

impl RunningInstruction {

    fn new(program: String) -> RunningInstruction {

        RunningInstruction { program }
    }
}
// --------------------------------------------------------------------------------------
