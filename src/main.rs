mod intruments;

#[allow(unused)]
use intruments::*;

use rodio::DeviceSinkBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.wav")?;
    player.append(rodio::Decoder::try_from(file)?);

    player.sleep_until_end();

    Ok(())
}
