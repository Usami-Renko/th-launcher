
use tui::layout::{ Rect, Alignment };
use tui::style::{ Style, Color, Modifier };
use tui::widgets::{ Block, Borders, Paragraph, SelectableList, Text, Widget };

use crate::scene::TerminalPainter;
use crate::config::ConfigOp;
use crate::config::tab::{ TabsConfig, TabConfig };

pub struct ContentPainter {

    state: ListState,
    current_tab: usize,

    tabs: Vec<TabConfig>,

    block: Block<'static>,
    style_selected: Style,
    style_unselect: Style,
}

struct ListState {

    index: Option<usize>,
    count: usize,
}

impl TerminalPainter for ContentPainter {

    fn draw(&mut self, f: &mut crate::DstFrame, area: Rect) {

        let dest_tab = &self.tabs[self.current_tab];

        self.block
            .title(&dest_tab.name)
            .render(f, area);

        if self.tabs[self.current_tab].items.is_empty() {
            self.draw_welcome(f, area.inner(1));
        } else {
            self.draw_game_list(f, &dest_tab, area.inner(1));
        }
    }
}

impl ContentPainter {

    pub fn new(config: TabsConfig) -> ContentPainter {

        ContentPainter {
            block: Block::default().borders(Borders::ALL),
            style_selected: Style::default().fg(Color::LightGreen).modifier(Modifier::Bold),
            style_unselect: Style::default().fg(Color::Gray),
            state: ListState { index: None, count: config.tabs[0].items.len() },
            current_tab: 0,
            tabs: config.tabs,
        }
    }

    pub fn set_tab(&mut self, index: usize) {
        self.current_tab = index;

        self.state.index = None;
        self.state.count = self.tabs[index].items.len();
    }

    pub fn update_tab(&mut self, ops: &ConfigOp) {

        match ops {
            | ConfigOp::AppendGame { tab_index, config } => {
                self.tabs[*tab_index].items.push(config.clone());
                self.set_tab(self.tabs.len() - 1)
            },
            | ConfigOp::RemoveGame { tab_index, item_index } => {
                self.tabs[*tab_index].items.remove(*item_index);
                self.set_tab(*tab_index)
            },
            | ConfigOp::AppendTab { config } => {
                self.tabs.push(config.clone());
                self.set_tab(self.tabs.len() - 1);
            },
            | ConfigOp::RemoveTab { tab_index } => {
                self.tabs.remove(*tab_index);
                // TODO: Handle situation if all the tabs were removed.
                self.set_tab(0);
            },
            | _ => {},
        }
    }

    pub fn draw_welcome(&self, f: &mut crate::DstFrame, area: Rect) {

        Paragraph::new([Text::raw("The game list is empty, please try to add a new game.")].iter())
            .alignment(Alignment::Left)
            .render(f, area);
    }

    pub fn draw_game_list(&self, f: &mut crate::DstFrame, tab: &TabConfig, area: Rect) {

        let games: Vec<&String> = tab.items.iter()
            .map(|item| &item.name).collect();

        SelectableList::default()
            .items(&games)
            .select(self.state.index)
            .style(self.style_unselect)
            .highlight_style(self.style_selected)
            .highlight_symbol(crate::config::manifest::HIGHLIGHT_SYMBOL)
            .render(f, area);
    }

    /// Launch current selected game.
    pub fn launch(&self) {
        unimplemented!()
    }

    pub fn next_tab(&mut self) {

        if self.tabs[self.current_tab].items.is_empty() == false {
            self.state.next();
        }
    }

    pub fn previous_tab(&mut self) {

        if self.tabs[self.current_tab].items.is_empty() == false {
            self.state.previous();
        }
    }
}

impl ListState {

    fn next(&mut self) {

        self.index = if let Some(index) = self.index {
            Some((index + 1) % self.count)
        } else {
            Some(0)
        };
    }

    fn previous(&mut self) {

        self.index = if let Some(index) = self.index {
            Some((index + self.count - 1) % self.count)
        } else {
            Some(self.count - 1)
        };
    }
}
