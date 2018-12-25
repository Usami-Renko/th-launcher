
use tui::layout::Rect;
use tui::widgets::{ Block, Borders, Widget };

use crate::scene::TerminalPainter;

pub struct NavTabPainter {

    block: Block<'static>,
}

impl NavTabPainter {

    pub fn new() -> NavTabPainter {

        let block = Block::default()
            .title("Navigation")
            .borders(Borders::ALL);

        NavTabPainter { block }
    }
}

impl TerminalPainter for NavTabPainter {

    fn draw(&mut self, f: &mut crate::DestFrame, area: Rect) {

        self.block.render(f, area);
    }
}
