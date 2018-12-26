
#[macro_use]
extern crate serde_derive;

mod utils;
mod scene;
mod config;

use termion::raw::{ IntoRawMode, RawTerminal };
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;

use tui::Terminal;
use tui::terminal::Frame;
use tui::backend::TermionBackend;

use crate::config::EngineConfig;
use crate::config::manifest::EXIT_KEY;

use std::io;

type THLError     = Result<(), failure::Error>;
type THLBackend   = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>;
type DstTerminal  = Terminal<THLBackend>;
type DstFrame<'a> = Frame<'a, THLBackend>;

fn init_terminal() -> Result<DstTerminal, failure::Error> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn main_loop(terminal: &mut DstTerminal, config: EngineConfig) -> THLError {

    let mut thl_scene = scene::THLScene::new();
    let event_dispatcher = utils::THLEvents::with_config(&config.setting);

    loop {
        terminal.draw(|mut f| {
            thl_scene.draw(&mut f);
        })?;

        match event_dispatcher.next()? {
            | utils::THLEvent::Input(key) => {
                if key == EXIT_KEY {
                    break
                }
            },
            | _ => {},
        }
    }

    Ok(())
}

fn main() -> THLError {

    // Read configuration
    let config = EngineConfig::init().unwrap_or_else(|| {
        let config = EngineConfig::default();
        config.write_manifest()
            .expect("Failed to write manifest content.");
        config
    });

    // Terminal initialization
    let mut terminal = init_terminal()?;
    main_loop(&mut terminal, config)?;

    Ok(())
}
