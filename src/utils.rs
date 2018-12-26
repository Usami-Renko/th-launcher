
use termion::event::Key;
use termion::input::TermRead;

use std::io;
use std::sync::mpsc;
use std::thread;

use crate::config::manifest::EXIT_KEY;
use crate::config::setting::SettingConfig;

pub enum THLEvent<I> {
    Input(I),
    Tick,
}

pub struct THLEvents {

    rx: mpsc::Receiver<THLEvent<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle : thread::JoinHandle<()>,
}

impl THLEvents {

    pub fn with_config(config: SettingConfig) -> THLEvents {

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

                        if key == EXIT_KEY {
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
