use rodio::cpal::{self, traits::HostTrait};

fn main() {
    let host = cpal::default_host();
    let devices = host.devices();

    for device in devices {}
    println!("Hello, world!");
}
