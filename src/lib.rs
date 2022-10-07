use std::{ffi::CStr, slice};

use librazermacos_sys::{
    closeAllRazerDevices, getAllRazerDevices, razer_attr_read_get_battery,
    razer_attr_read_is_charging,
};

mod devices;

pub struct RazerDevices(librazermacos_sys::RazerDevices);

impl RazerDevices {
    pub fn all() -> Self {
        let devices = unsafe { getAllRazerDevices() };
        assert!(!devices.devices.is_null());

        Self(devices)
    }

    pub fn find(&mut self, product_id: u16) -> Option<RazerDevice<'_>> {
        self.slice_mut()
            .iter_mut()
            .find(|device| device.productId == product_id)
            .map(RazerDevice)
    }

    fn slice_mut(&mut self) -> &mut [librazermacos_sys::RazerDevice] {
        unsafe { slice::from_raw_parts_mut(self.0.devices, self.0.size as usize) }
    }
}

impl Drop for RazerDevices {
    fn drop(&mut self) {
        unsafe { closeAllRazerDevices(self.0) }
    }
}

#[derive(Debug)]
pub struct RazerDevice<'a>(&'a mut librazermacos_sys::RazerDevice);

impl RazerDevice<'_> {
    pub fn battery(&self) -> u8 {
        let c_str = unsafe {
            let mut buf = [0i8; 4];
            razer_attr_read_get_battery(self.0.usbDevice, buf.as_mut_ptr());

            CStr::from_ptr(buf.as_ptr())
        };
        let str = format!("{}", c_str.to_str().unwrap()).trim().to_string();
        let current: u8 = str.parse().unwrap();
        return ((current as f32 / 255.0) * 100.0) as u8;
    }

    pub fn is_charging(&self) -> bool {
        let c_str = unsafe {
            let mut buf = [0i8; 4];
            razer_attr_read_is_charging(self.0.usbDevice, buf.as_mut_ptr());

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
        let mut devices = RazerDevices::all();
        let device = devices.find(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16);
        if let Some(device) = device {
            println!("Battery: {}", device.battery());
        } else {
            println!("Device not found");
        }
    }
}
