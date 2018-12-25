
use tui::layout::Rect;
use tui::widgets::{ Block, Borders, Widget };

use crate::scene::TerminalPainter;

pub struct OperationPainter {

    block: Block<'static>,
}

impl OperationPainter {

    pub fn new() -> OperationPainter {

        let block = Block::default()
            .title("Instruction")
            .borders(Borders::ALL);

        OperationPainter { block }
    }
}

impl TerminalPainter for OperationPainter {

    fn draw(&mut self, f: &mut crate::DestFrame, area: Rect) {

        self.block.render(f, area);
    }
}
