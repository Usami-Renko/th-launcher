
use tui::layout::Rect;
use tui::widgets::{ Block, Borders, Widget };

use crate::scene::TerminalPainter;

pub struct ContentPainter {

    block: Block<'static>,
}

impl ContentPainter {

    pub fn new() -> ContentPainter {

        let block = Block::default()
            .title("Main")
            .borders(Borders::ALL);

        ContentPainter { block }
    }
}

impl TerminalPainter for ContentPainter {

    fn draw(&mut self, f: &mut crate::DstFrame, area: Rect) {

        self.block.render(f, area);
    }
}
