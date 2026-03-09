use rodio::{MixerDeviceSink, Player};
use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct Piano {
    pub(crate) audio_player: Player,
    pub(crate) output_stream: MixerDeviceSink,
    //output_stream_handle: Output,
    note_key_map: Option<HashMap<char, Vec<u8>>>,
}

impl Piano {
    pub(crate) fn new(output_stream: MixerDeviceSink) -> Self {
        let player = Player::connect_new(output_stream.mixer());
        Piano {
            output_stream: output_stream,
            audio_player: player,
            note_key_map: None,
        }
    }
}
