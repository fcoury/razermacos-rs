use librazermacos_sys::USB_DEVICE_ID_RAZER_MOUSE_DOCK_PRO;
use razermacos::RazerDevices;

fn main() {
    let mut devices = RazerDevices::new();
    let device = devices.find(USB_DEVICE_ID_RAZER_MOUSE_DOCK_PRO as u16);
    if let Some(device) = device {
        let ch_str = if device.is_charging() { " ⚡️" } else { "" };
        println!("Battery: 🔋{}%{}", device.battery(), ch_str);
    } else {
        println!("Device not found");
    }
}
