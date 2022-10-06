use librazermacos_sys::{RazerDevice, USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS};

fn main() {
    let device = RazerDevice::find(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16);
    if let Some(device) = device {
        println!("Battery: {}", device.battery());
    } else {
        println!("Device not found");
    }
}
