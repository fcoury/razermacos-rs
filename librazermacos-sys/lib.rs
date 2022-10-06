#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use ffi::razer_attr_read_get_battery;
use std::{mem::MaybeUninit, slice};

pub use ffi::getAllRazerDevices;
pub use ffi::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS;

use crate::ffi::closeAllRazerDevices;

// #[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Razer;

#[derive(Clone, Debug)]
pub struct RazerDevice {
    pub internal_device_id: i32,
    pub product_id: u16,
}

impl Razer {
    pub fn get_all_devices(&self) -> Vec<RazerDevice> {
        let devices = unsafe {
            let devices = getAllRazerDevices();
            assert!(!devices.devices.is_null());
            closeAllRazerDevices(devices);
            slice::from_raw_parts(devices.devices, devices.size as usize)
        };

        devices
            .iter()
            .map(|device| RazerDevice {
                internal_device_id: device.internalDeviceId,
                product_id: device.productId,
            })
            .collect::<Vec<_>>()
    }

    pub fn battery(&self, device: &RazerDevice) -> i8 {
        unsafe {
            let devices = getAllRazerDevices();
            let slice = slice::from_raw_parts(devices.devices, devices.size as usize);

            let device = slice
                .iter()
                .find(|d| d.internalDeviceId == device.internal_device_id)
                .unwrap();

            let mut buf = MaybeUninit::uninit();
            razer_attr_read_get_battery(device.usbDevice, buf.as_mut_ptr());
            closeAllRazerDevices(devices);

            let buf = buf.assume_init();
            buf.to_owned()
        }
    }
}

impl RazerDevice {
    pub fn battery(&self) -> i8 {
        Razer {}.battery(&self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let razer = Razer {};
        let devices = razer.get_all_devices();
        println!("Devices: {:?}", devices);
        for device in devices {
            if device.product_id == USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16 {
                println!("Found Viper Ultimate Wireless");
                println!("Battery: {}", device.battery());
            }
        }
    }
}
