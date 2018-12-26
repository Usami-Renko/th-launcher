
use tui::layout::Rect;
use tui::style::{ Color, Style };
use tui::widgets::{ Block, Tabs, Borders, Widget };

use crate::scene::TerminalPainter;
use crate::config::ConfigOp;
use crate::config::tab::TabsConfig;

pub struct NavTabPainter {

    pub state: TabsState,

    titles : Vec<String>,
    block  : Block<'static>,

    style_selected: Style,
    style_unselect: Style,
}

pub struct TabsState {

    index: usize,
    count: usize,
}

impl TerminalPainter for NavTabPainter {

    fn draw(&mut self, f: &mut crate::DstFrame, area: Rect) {

        Tabs::default()
            .block(self.block)
            .titles(&self.titles)
            .select(self.state.index)
            .style(self.style_unselect)
            .highlight_style(self.style_selected)
            .render(f, area);
    }
}

impl NavTabPainter {

    pub fn new(config: &TabsConfig) -> NavTabPainter {

        let block = Block::default()
            .title("Navigation")
            .borders(Borders::ALL);

        let state = TabsState {
            index: 0,
            count: config.tabs.len(),
        };

        let titles = config.tabs.iter()
            .map(|tab| tab.name.clone()).collect();

        NavTabPainter {
            state, block, titles,
            style_selected: Style::default().fg(Color::Cyan),
            style_unselect: Style::default().fg(Color::Yellow),
        }
    }

    pub fn current_index(&self) -> usize {
        self.state.index
    }

    pub fn update_tabs(&mut self, ops: &ConfigOp) {
        match ops {
            | ConfigOp::AppendTab { config } => {
                self.titles.push(config.name.clone());
                self.state.reset(self.titles.len() - 1, self.state.count + 1);
            },
            | ConfigOp::RemoveTab { tab_index } => {
                self.titles.remove(*tab_index);
                // TODO: Handle situation if all the tabs were removed.
                self.state.reset(0, self.state.count - 1);
            },
            | _ => {},
        }
    }
}

impl TabsState {

    pub fn reset(&mut self, index: usize, count: usize) {
        self.index = index;
        self.count = count;
    }

    pub fn next(&mut self) {

        self.index = (self.index + 1) % self.count;
    }

    pub fn previous(&mut self) {

        self.index = (self.index + self.count - 1) % self.count;
    }
}
