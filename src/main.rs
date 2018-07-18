extern crate sonos;

fn main() {
    let devices = sonos::discover().unwrap();

    for device in devices.iter() {
        let volume = device.volume().unwrap();
        let track = device.track().unwrap();
        println!("Found device {} playing {} - {} at {}% volume.", device.name, track.title, track.artist, volume);
    }
}
