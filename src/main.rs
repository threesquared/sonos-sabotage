#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate lazy_static;
extern crate galvanic_mock;
extern crate clap;
extern crate sonos;
extern crate rand;
extern crate regex;

use std::collections::HashMap;
use std::sync::Mutex;
use std::net::IpAddr;
use std::time::Duration;
use std::thread;
use clap::{App, Arg};
use rand::prelude::*;
use regex::Regex;
use galvanic_mock::{mockable, use_mocks};

#[derive(Clone)]
pub struct SpeakerState {
    pub volume: u8,
}

pub struct SonosSpeaker<'a> {
    pub sonos: &'a sonos::Speaker,
}

#[mockable]
pub trait SpeakerTrait {
    fn pause(&self) -> Result<(), sonos::Error>;
    fn stop(&self) -> Result<(), sonos::Error>;
    fn next(&self) -> Result<(), sonos::Error>;
    fn clear_queue(&self) -> Result<(), sonos::Error>;
    fn play_track(&self, uri: &str) -> Result<(), sonos::Error>;
    fn volume(&self) -> Result<u8, sonos::Error>;
    fn set_volume(&self, volume: u8) -> Result<(), sonos::Error>;
    fn mute(&self) -> Result<(), sonos::Error>;
    fn transport_state(&self) -> Result<sonos::TransportState, sonos::Error>;
    fn track(&self) -> Result<sonos::Track, sonos::Error>;
}

impl<'a> SpeakerTrait for SonosSpeaker<'a> {
    fn pause(&self) -> Result<(), sonos::Error> {
        self.sonos.pause()
    }
    fn stop(&self) -> Result<(), sonos::Error> {
        self.sonos.stop()
    }
    fn next(&self) -> Result<(), sonos::Error> {
        self.sonos.next()
    }
    fn clear_queue(&self) -> Result<(), sonos::Error> {
        self.sonos.clear_queue()
    }
    fn play_track(&self, uri: &str) -> Result<(), sonos::Error> {
        self.sonos.play_track(uri)
    }
    fn volume(&self) -> Result<u8, sonos::Error> {
        self.sonos.volume()
    }
    fn set_volume(&self, volume: u8) -> Result<(), sonos::Error> {
        self.sonos.set_volume(volume)
    }
    fn mute(&self) -> Result<(), sonos::Error> {
        self.sonos.mute()
    }
    fn transport_state(&self) -> Result<sonos::TransportState, sonos::Error> {
        self.sonos.transport_state()
    }
    fn track(&self) -> Result<sonos::Track, sonos::Error> {
        self.sonos.track()
    }
}

lazy_static! {
    static ref DEVICES: Mutex<Vec<sonos::Speaker>> = Mutex::new(Vec::new());
    static ref STATES: Mutex<HashMap<std::net::IpAddr, SpeakerState>> = Mutex::new(HashMap::new());
}

fn get_state(ip: std::net::IpAddr) -> Option<SpeakerState> {
    STATES.lock().unwrap().get(&ip).cloned()
}

fn set_state(ip: std::net::IpAddr, state: SpeakerState) {
    STATES.lock().unwrap().insert(ip, state);
}

fn main() {
    let matches = App::new("sonos-sabotage")
        .arg(Arg::with_name("interval")
            .help("The interval to check devices in ms")
            .short("i")
            .default_value("5000")
            .takes_value(true)
        )
        .arg(Arg::with_name("pattern")
            .help("Pattern to match in assassin mode")
            .short("p")
            .default_value("Ed Sheeran")
            .takes_value(true)
        )
        .arg(Arg::with_name("uri")
            .help("The URI to play in dictator mode")
            .short("u")
            .default_value("x-sonos-spotify:spotify:track:1wsRitfRRtWyEapl0q22o8")
            .takes_value(true)
        )
        .arg(Arg::with_name("ip")
            .help("Device IP address to target")
            .short("x")
            .takes_value(true)
        )
        .arg(Arg::with_name("devices")
            .help("Print out all devices found on the current network and exit")
            .short("y")
            .long("devices")
        )
        .arg(Arg::with_name("percent")
            .help("Percent of the time saboteur mode should take an action")
            .short("z")
            .default_value("5")
            .long("percent")
        )
        .arg(Arg::with_name("oldman")
             .help("This mode detects volume increases on devices and turns them down")
             .short("o")
             .long("oldman")
        )
        .arg(Arg::with_name("assassin")
            .help("This mode matches specific tracks and eliminates them")
            .short("a")
            .long("assassin")
            .conflicts_with_all(&[
                "dictator",
                "totalitarian"
            ])
        )
        .arg(Arg::with_name("dictator")
            .help("This mode enforces a specific track to be playing")
            .short("d")
            .long("dictator")
            .conflicts_with_all(&[
                "saboteur",
                "totalitarian"
            ])
        )
        .arg(Arg::with_name("saboteur")
            .help("This mode aims to completely disrupt playback")
            .short("s")
            .long("saboteur")
        )
        .arg(Arg::with_name("totalitarian")
            .help("This mode clears all queues and stops playing tracks")
            .short("t")
            .long("totalitarian")
        )
        .get_matches();

    thread::spawn(|| {
        loop {
            discover_devices();

            thread::sleep(Duration::from_millis(30000));
        }
    });

    let check_interval = matches.value_of("interval").unwrap();

    loop {
        let devices = DEVICES.lock().unwrap();

        for device in devices.iter() {
            println!("Checking device {} at {}", device.name, device.ip);

            if matches.is_present("ip") {
                let ip = matches.value_of("ip").unwrap().parse::<IpAddr>().unwrap();
                if device.ip != ip {
                    return
                }
            }

            let previous_state = get_state(device.ip);

            let sonos = SonosSpeaker {
                sonos: device,
            };

            if matches.is_present("oldman") {
                old_man(&sonos, previous_state);
            }

            if matches.is_present("assassin") {
                assassin(&sonos, matches.value_of("pattern").unwrap());
            }

            if matches.is_present("dictator") {
                dictator(&sonos, matches.value_of("uri").unwrap());
            }

            if matches.is_present("saboteur") {
                saboteur(&sonos, matches.value_of("percent").unwrap());
            }

            if matches.is_present("totalitarian") {
                totalitarian(&sonos);
            }

            set_state(device.ip, SpeakerState {
                volume: device.volume().unwrap(),
            });
        }

        thread::sleep(Duration::from_millis(check_interval.parse::<u64>().unwrap()));
    }
}

fn discover_devices() {
    println!("Scanning for Sonos devices...");

    let mut device_state = DEVICES.lock().expect("Could not lock device mutex");
    let devices = sonos::discover().unwrap();

    if devices.len() == 0 {
        println!("No devices found!");
        return;
    }

    println!("Found {} devices", devices.len());

    *device_state = devices;
}

fn old_man(device: &SpeakerTrait, previous_state: std::option::Option<SpeakerState>) {
    if previous_state.is_some() {
        let current_volume = device.volume().unwrap();
        let previous_volume = previous_state.unwrap().volume;

        if current_volume > previous_volume {
            let difference: u8 = current_volume - previous_volume;

            // TODO: Accept these params as arguments
            if difference > 5 {
                let reduction: i8 = current_volume as i8 - (difference as f32 * 1.3) as i8;

                if reduction > 0 {
                    println!("Detected volume increase of {} points! Decreasing to {}", difference, reduction);
                    device.set_volume(reduction as u8).unwrap();
                }
            }
        }
    }
}

fn assassin(device: &SpeakerTrait, pattern: &str) {
    if let Ok(track) = device.track() {
        let regex = Regex::new(pattern.trim()).unwrap();

        if regex.is_match(&track.title) || regex.is_match(&track.artist) {
            // TODO: Optional subtle fade out and skip mode
            println!("Detected matched track! Assassinating {} {}", track.title, track.artist);

            match device.next() {
                Ok(_) => println!("Skipped to next track in the queue"),
                Err(_) => {
                    device.stop().unwrap();
                    println!("Could not skip, stopping playback")
                }
            }
        }
    }
}

fn dictator(device: &SpeakerTrait, uri: &str) {
    if let Ok(track) = device.track() {
        if track.uri == uri {
            return
        }
        println!("Device playing forbidden track! {} {}", track.uri, uri);

        device.clear_queue().unwrap();

        match device.play_track(uri) {
            Ok(_) => println!("Corrected it"),
            Err(err) => println!("Could not dictate track, {}", err)
        }
    }
}

fn saboteur(device: &SpeakerTrait, percent: &str) {
    let mut rng = thread_rng();
    let action_chance = rng.gen_range(0, 100);

    if action_chance > percent.parse::<u32>().unwrap() {
        let action_choice = rng.gen_range(1, 4);

        // TODO: Add more choices and configurable weighted randomness
        match action_choice {
            1 => device.mute().unwrap(),
            2 => device.next().unwrap(),
            3 => device.set_volume(rng.gen_range(0, 100)).unwrap(),
            4 => device.pause().unwrap(),
            _ => {},
        }

        println!("Sabotage device with {}", action_choice);
    }
}

fn totalitarian(device: &SpeakerTrait) {
    let transport_state = device.transport_state().unwrap();

    if transport_state == sonos::TransportState::Playing {
        println!("Device is active, shutting it down");

        device.stop().unwrap();
        device.clear_queue().unwrap();
    }
}

#[cfg(test)]
#[use_mocks]
mod tests {
    use super::*;

    #[test]
    fn test_old_man() {
        let mock = new_mock!(SpeakerTrait);

        given! {
            <mock as SpeakerTrait>::volume() then_return Ok(11) always;
            <mock as SpeakerTrait>::set_volume() then_return Ok(()) always;
        }

        expect_interactions! {
            <mock as SpeakerTrait>::set_volume() times 1;
        }

        old_man(&mock, Some(SpeakerState {
            volume: 5,
        }));
    }

    #[test]
    fn test_assassin() {
        let mock = new_mock!(SpeakerTrait);

        given! {
            <mock as SpeakerTrait>::track() then_return Ok(sonos::Track {
                title: "testPattern".to_string(),
                artist: "testPattern".to_string(),
                album: "test".to_string(),
                queue_position: 1,
                uri: "test".to_string(),
                duration: Duration::from_secs(300),
                running_time: Duration::from_secs(100)
            }) always;
            <mock as SpeakerTrait>::next() then_return Ok(()) always;
        }

        expect_interactions! {
            <mock as SpeakerTrait>::next() times 1;
        }

        assassin(&mock, "testPattern");
    }

    #[test]
    fn test_dictator() {
        let mock = new_mock!(SpeakerTrait);

        given! {
            <mock as SpeakerTrait>::track() then_return Ok(sonos::Track {
                title: "test".to_string(),
                artist: "test".to_string(),
                album: "test".to_string(),
                queue_position: 1,
                uri: "test".to_string(),
                duration: Duration::from_secs(300),
                running_time: Duration::from_secs(100)
            }) always;
            <mock as SpeakerTrait>::clear_queue() then_return Ok(()) always;
            <mock as SpeakerTrait>::play_track() then_return Ok(()) always;
        }

        expect_interactions! {
            <mock as SpeakerTrait>::clear_queue() times 1;
            <mock as SpeakerTrait>::play_track() times 1;
        }

        dictator(&mock, "testURI");
    }

    #[test]
    fn test_totalitarian() {
        let mock = new_mock!(SpeakerTrait);

        given! {
            <mock as SpeakerTrait>::transport_state() then_return Ok(sonos::TransportState::Playing) always;
            <mock as SpeakerTrait>::stop() then_return Ok(()) always;
            <mock as SpeakerTrait>::clear_queue() then_return Ok(()) always;
        }

        expect_interactions! {
            <mock as SpeakerTrait>::stop() times 1;
            <mock as SpeakerTrait>::clear_queue() times 1;
        }

        totalitarian(&mock);
    }
}
