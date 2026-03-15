use crossterm::event::{poll, read, Event, KeyCode};
use rodio::{MixerDeviceSink, Player};
use std::{collections::HashMap, time::Duration};

use super::Instrument;

#[allow(dead_code)]
pub(crate) struct Piano {
    pub(crate) audio_player: Player,
    pub(crate) output_stream: MixerDeviceSink,
    //output_stream_handle: Output,
    note_key_map: Option<HashMap<char, Vec<u8>>>,
}

impl Instrument for Piano {
    fn run(&mut self) -> () {
        loop {
            if poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().expect("Couldn't read key") {
                    if event.kind == crossterm::event::KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char(key) => self.handle_key(key),
                            KeyCode::Esc => break,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn load_notes(&self) -> Option<HashMap<char, Vec<u8>>> {
        None
    }
}

impl Piano {
    pub(crate) fn new(output_stream: MixerDeviceSink) -> Self {
        let player = Player::connect_new(output_stream.mixer());
        let mut piano = Piano {
            output_stream: output_stream,
            audio_player: player,
            note_key_map: None,
        };

        piano.note_key_map = piano.load_notes();

        piano
    }

    pub(crate) fn handle_key(&mut self, key: char) -> () {
        if let Some(ref keymap) = self.note_key_map {
            if let Some(_source) = keymap.get(&key) {
                todo!()
            }
        }
    }
}
