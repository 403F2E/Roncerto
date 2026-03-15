use super::Instrument;
use rodio::{MixerDeviceSink, Player};

use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct Guitar {
    pub(crate) audio_player: Player,
    pub(crate) output_stream: MixerDeviceSink,
    //output_stream_handle: Output,
    note_key_map: Option<HashMap<char, Vec<u8>>>,
}

impl Guitar {
    pub(crate) fn new(output_stream: MixerDeviceSink) -> Self {
        let player = Player::connect_new(output_stream.mixer());
        let mut guitar = Guitar {
            output_stream: output_stream,
            audio_player: player,
            note_key_map: None,
        };

        guitar.note_key_map = guitar.load_notes();

        guitar
    }
}

impl Instrument for Guitar {
    fn run(&mut self) -> () {
        todo!()
    }

    fn load_notes(&self) -> Option<HashMap<char, Vec<u8>>> {
        None
    }
}
