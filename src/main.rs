
mod utils;
mod scene;

use termion::raw::{ IntoRawMode, RawTerminal };
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;

use tui::Terminal;
use tui::terminal::Frame;
use tui::backend::TermionBackend;

use std::io;

type THLError = Result<(), failure::Error>;
type THLBackend = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>;
type DestTerminal = Terminal<THLBackend>;
type DestFrame<'a> = Frame<'a, THLBackend>;

fn init_terminal() -> Result<DestTerminal, failure::Error> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn main_loop(terminal: &mut DestTerminal, config: utils::THLConfig) -> THLError {

    let mut thl_scene = scene::THLScene::new();
    let event_dispatcher = utils::THLEvents::with_config(config.clone());

    loop {
        terminal.draw(|mut f| {
            thl_scene.draw(&mut f);
        })?;

        match event_dispatcher.next()? {
            | utils::THLEvent::Input(key) => {
                if key == config.exit_key {
                    break
                }
            },
            | _ => {},
        }
    }

    Ok(())
}

fn main() -> THLError {

    // Terminal initialization
    let mut terminal = init_terminal()?;
    main_loop(&mut terminal, utils::THLConfig::default())?;

    Ok(())
}
