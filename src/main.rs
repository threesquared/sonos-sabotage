#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate sonos;
extern crate schedule_recv;

use std::collections::HashMap;
use std::sync::Mutex;
use clap::{App, Arg};

#[derive(Clone)]
pub struct SpeakerState {
    pub volume: u8,
}

lazy_static! {
    static ref DEVICES: Mutex<HashMap<std::net::IpAddr, SpeakerState>> = Mutex::new(HashMap::new());
}

fn get_previous_state(ip: std::net::IpAddr) -> Option<SpeakerState> {
    DEVICES.lock().unwrap().get(&ip).cloned()
}

fn main() {
    let matches = App::new("stuxsonos")
        .version("0.1.0")
        .arg(Arg::with_name("interval")
            .help("The refresh interval to poll devices in ms")
            .short("i")
            .default_value("10000")
            .takes_value(true)
        )
        .arg(Arg::with_name("oldMan")
             .help("This mode detects volume increases on devices and turns them down")
             .short("o")
             .long("oldMan")
        )
        .get_matches();

    let interval = matches.value_of("interval").unwrap();
    let tick = schedule_recv::periodic_ms(interval.parse::<u32>().unwrap());

    loop {
        tick.recv().unwrap();

        println!("Checking for Sonos devices...");

        let devices = sonos::discover().unwrap();

        if devices.len() == 0 {
            println!("No devices found!");
            return;
        }

        for device in devices.iter() {
            let current_volume = device.volume().unwrap();
            println!("Found device {} at {} at {}% volume.", device.name, device.ip, current_volume);

            let previous_state = get_previous_state(device.ip);

            if matches.is_present("oldMan") {
                println!("Old man mode is turned on");

                if previous_state.is_some() {
                    let previous_volume = previous_state.unwrap().volume;
                    println!("Old state {}", previous_volume);
                }
            }

            DEVICES.lock().unwrap().insert(device.ip, SpeakerState {
                volume: current_volume,
            });
        }
    }
}
