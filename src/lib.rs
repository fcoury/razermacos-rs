use std::{ffi::CStr, mem::MaybeUninit, slice};

use librazermacos_sys::{
    closeAllRazerDevices, getAllRazerDevices, razer_attr_read_get_battery,
    razer_attr_read_is_charging,
};

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

    pub fn battery(&self) -> i8 {
        unsafe {
            let devices = getAllRazerDevices();
            let slice = slice::from_raw_parts(devices.devices, devices.size as usize);

            let device = slice
                .iter()
                .find(|d| d.internalDeviceId == self.internal_device_id)
                .unwrap();

            let mut buf = MaybeUninit::uninit();
            razer_attr_read_get_battery(device.usbDevice, buf.as_mut_ptr());
            closeAllRazerDevices(devices);

            let buf = buf.assume_init();
            buf.to_owned()
        }
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
