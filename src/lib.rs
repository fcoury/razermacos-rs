use std::{ffi::CStr, slice};

use librazermacos_sys::{
    closeAllRazerDevices, getAllRazerDevices, razer_attr_read_get_battery,
    razer_attr_read_is_charging,
};

// Device constants
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ABYSSUS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ABYSSUS_1800;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ABYSSUS_2000;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ABYSSUS_ELITE_DVA_EDITION;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ABYSSUS_ESSENTIAL;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ABYSSUS_V2;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_ATHERIS_RECEIVER;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_BASILISK;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_BASILISK_ESSENTIAL;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_BASILISK_ULTIMATE;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_BASILISK_ULTIMATE_RECEIVER;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_BASILISK_V2;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_BASILISK_V3;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_1800;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_2013;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_3500;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_3_5G;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_CHROMA;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_ELITE;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_ESSENTIAL;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_ESSENTIAL_2021;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_ESSENTIAL_WHITE_EDITION;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_V2;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_V2_MINI;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_V2_PRO_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DEATHADDER_V2_PRO_WIRELESS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_DIAMONDBACK_CHROMA;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_IMPERATOR;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_LANCEHEAD_TE_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_LANCEHEAD_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_LANCEHEAD_WIRELESS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_LANCEHEAD_WIRELESS_RECEIVER;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_LANCEHEAD_WIRELESS_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_2012_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_2012_WIRELESS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_ELITE;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_TE_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_WIRELESS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_WIRELESS_RECEIVER;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_MAMBA_WIRELESS_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_2012;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_2014;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_CHROMA;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_HEX;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_HEX_RED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_HEX_V2;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_LEFT_HANDED_2020;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_PRO_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_PRO_WIRELESS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_NAGA_TRINITY;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OROCHI_2011;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OROCHI_2013;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OROCHI_CHROMA;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OROCHI_V2;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OROCHI_V2_BLUETOOTH;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OROCHI_V2_RECEIVER;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_OUROBOROS;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_TAIPAN;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER_8KHZ;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER_MINI;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRED;
pub use librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS;

#[derive(Clone, Debug)]
pub struct RazerDevice {
    pub internal_device_id: i32,
    pub product_id: u16,
}

impl RazerDevice {
    pub fn all() -> Vec<RazerDevice> {
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

    pub fn find(product_id: u16) -> Option<RazerDevice> {
        Self::all()
            .into_iter()
            .find(|device| device.product_id == product_id)
    }

    pub fn battery(&self) -> u8 {
        let c_str = unsafe {
            let devices = getAllRazerDevices();
            let slice = slice::from_raw_parts(devices.devices, devices.size as usize);

            let device = slice
                .iter()
                .find(|d| d.internalDeviceId == self.internal_device_id)
                .unwrap();

            let mut buf = [0i8; 4];
            razer_attr_read_get_battery(device.usbDevice, buf.as_mut_ptr());
            closeAllRazerDevices(devices);

            CStr::from_ptr(buf.as_ptr())
        };
        let str = format!("{}", c_str.to_str().unwrap()).trim().to_string();
        let current: u8 = str.parse().unwrap();
        return ((current as f32 / 255.0) * 100.0) as u8;
    }

    pub fn is_charging(&self) -> bool {
        let c_str = unsafe {
            let devices = getAllRazerDevices();
            let slice = slice::from_raw_parts(devices.devices, devices.size as usize);

            let device = slice
                .iter()
                .find(|d| d.internalDeviceId == self.internal_device_id)
                .unwrap();

            let mut buf = [0i8; 4];
            razer_attr_read_is_charging(device.usbDevice, buf.as_mut_ptr());
            closeAllRazerDevices(devices);

            CStr::from_ptr(buf.as_ptr())
        };
        format!("{}", c_str.to_str().unwrap()).starts_with("1")
    }
}

#[cfg(test)]
mod tests {
    use librazermacos_sys::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS;

    use super::*;

    #[test]
    fn it_works() {
        let device = RazerDevice::find(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16);
        if let Some(device) = device {
            println!("Battery: {}", device.battery());
        } else {
            println!("Device not found");
        }
    }
}
