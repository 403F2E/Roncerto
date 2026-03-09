mod intruments;

use intruments::Piano;

use clap::{ArgMatches, Command};

use rodio::DeviceSinkBuilder;
use std::error::Error;

fn config_arg() -> Command {
    Command::new("Roncerto")
        .about(
            "Roncerto is a Rust small emulator of piano and guitar (more intruments in the future)",
        )
        .author("403f2e")
        .version("0.1")
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream_handle = DeviceSinkBuilder::open_default_sink()?;
    stream_handle.log_on_drop(false);
    let matches: ArgMatches = config_arg().get_matches();
    let _intrument: Option<Piano> = if matches.contains_id("piano") {
        // Piano initialization here
        Some(Piano::new(stream_handle))
    } else {
        // Guitar initialization here
        None
    };

    // start the intrument
    // intruments.run();

    Ok(())
}
