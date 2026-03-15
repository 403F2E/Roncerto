mod intruments;

use intruments::{Guitar, Instrument, Piano};

use clap::{Arg, ArgMatches, Command};

use rodio::DeviceSinkBuilder;
use std::error::Error;

fn config_arg() -> Command {
    Command::new("Roncerto")
        .about(
            "Roncerto is a Rust small emulator of piano and guitar (more intruments in the future)",
        )
        .author("403f2e")
        .version("0.1")
        .arg(
            Arg::new("instrument")
                .short('i')
                .long("instrument")
                .value_name("INSTRUMENT")
                .help("Choose instrument: piano or guitar")
                .default_value("piano")
                .value_parser(["piano", "guitar"]),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream_handle = DeviceSinkBuilder::open_default_sink()?;
    stream_handle.log_on_drop(false);

    // Create an Instrument (Piano/Guitar)
    let matches: ArgMatches = config_arg().get_matches();
    let mut intrument: Box<dyn Instrument> = if "piano"
        == matches
            .get_one::<String>("instrument")
            .map(String::as_str)
            .unwrap_or("piano")
    {
        // Piano initialization here
        Box::new(Piano::new(stream_handle))
    } else {
        // Guitar initialization here
        Box::new(Guitar::new(stream_handle))
    };

    // start the intrument
    intrument.run();

    Ok(())
}
