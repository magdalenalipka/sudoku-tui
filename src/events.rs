// source:
// https://github.com/MitchelPaulin/sudoku-rs/blob/main/src/events.rs

use std::{io, sync::mpsc, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub const TICK_RATE_MS: u64 = 250;

pub enum Event<I> {
    Input(I),
    Tick,
}

// A small event handler that wrap termion input and tick events. Each event
// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _input_handle: thread::JoinHandle<()>,
    _tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    tick_rate: Duration,
}

impl Config {
    fn new() -> Self {
        Self {
            tick_rate: Duration::from_millis(TICK_RATE_MS),
        }
    }
}

impl Events {
    pub fn new() -> Self {
        Self::with_config(Config::new())
    }

    pub fn with_config(config: Config) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            rx,
            _input_handle: {
                let tx = tx.clone();
                thread::spawn(move || {
                    let stdin = io::stdin();
                    for key in stdin.keys().flatten() {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                })
            },
            _tick_handle: {
                thread::spawn(move || loop {
                    if tx.send(Event::Tick).is_err() {
                        break;
                    }
                    thread::sleep(config.tick_rate);
                })
            },
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
