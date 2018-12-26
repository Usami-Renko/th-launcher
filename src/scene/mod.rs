
mod navtab;
mod content;
mod ops;

use tui::layout::{ Constraint, Direction, Layout, Rect };

use crate::scene::navtab::NavTabPainter;
use crate::scene::content::ContentPainter;
use crate::scene::ops::OperationPainter;

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

    pub fn new() -> THLScene {

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(50),
                Constraint::Percentage(30),
            ].as_ref());

        THLScene {
            layout : chunks,
            navtab : NavTabPainter::new(),
            content: ContentPainter::new(),
            ops    : OperationPainter::new(),
        }
    }

    pub fn draw(&mut self, f: &mut crate::DstFrame) {

        let chunks = self.layout.clone()
            .split(f.size());

        self.navtab.draw(f, chunks[0]);
        self.content.draw(f, chunks[1]);
        self.ops.draw(f, chunks[2]);
    }
}
