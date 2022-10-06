use librazermacos_sys::{RazerDevice, USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS};

fn main() {
    let device = RazerDevice::find(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16);
    if let Some(device) = device {
        let ch_str = if device.is_charging() { " ⚡️" } else { "" };
        println!("Battery: 🔋{}%{}", device.battery(), ch_str);
    } else {
        println!("Device not found");
    }
}
