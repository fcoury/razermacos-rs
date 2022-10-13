use razermacos::RazerDevices;

fn main() {
    let mut devices = RazerDevices::new();
    match devices.all() {
        Some(devices) => devices
            .iter()
            .for_each(|device| println!("- {}", device.name)),
        None => (),
    };
}
