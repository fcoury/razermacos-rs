use librazermacos_sys::Razer;

fn main() {
    let razer = Razer {};
    let devices = razer.get_all_devices();
    for device in devices {
        if device.product_id
            == librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16
        {
            println!("Found Viper Ultimate Wireless");
            println!("Battery: {}", device.battery());
        }
    }
}
