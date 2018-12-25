
use termion::event::Key;
use termion::input::TermRead;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum THLEvent<I> {
    Input(I),
    Tick,
}

pub struct THLEvents {

    rx: mpsc::Receiver<THLEvent<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle : thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct THLConfig {

    pub exit_key : Key,
    pub tick_rate: Duration,
}

impl Default for THLConfig {

    fn default() -> THLConfig {

        THLConfig {
            exit_key : Key::Esc,
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl THLEvents {

    #[allow(dead_code)]
    pub fn new() -> THLEvents {

        THLEvents::with_config(THLConfig::default())
    }

    pub fn with_config(config: THLConfig) -> THLEvents {

        let (tx, rx) = mpsc::channel();

        let input_handle = {

            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {

                    if let Ok(key) = evt {
                        if let Err(_) = tx.send(THLEvent::Input(key)) {
                            return
                        }

                        if key == config.exit_key {
                            return
                        }
                    }
                }
            })
        };

        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {

                // TODO: Figure it why clone here.
                let tx = tx.clone();
                loop {
                    tx.send(THLEvent::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };

        THLEvents {
            rx, input_handle, tick_handle,
        }
    }

    pub fn next(&self) -> Result<THLEvent<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
