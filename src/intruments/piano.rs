use rodio::{MixerDeviceSink, Player};
use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct Piano {
    pub(crate) audio_player: Player,
    pub(crate) output_stream: MixerDeviceSink,
    //output_stream_handle: Output,
    is_playing: bool,
    note_key_map: Option<HashMap<char, Vec<u8>>>,
}
