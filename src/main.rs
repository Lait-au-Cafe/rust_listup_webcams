use std::ptr;
use windows::{
    core::*, 
    Win32::Media::MediaFoundation::*, 
};

fn main() -> Result<()>  {
    unsafe {
        let mut imf_attr_opt: Option<IMFAttributes> = None;
        MFCreateAttributes(&mut imf_attr_opt, 1)?;
        let imf_attr = imf_attr_opt.unwrap();
        println!("Set GUID. ");
        imf_attr.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID)?;
        
        let mut devices_opt_ptr: *mut Option<IMFActivate> = ptr::null_mut();
        let mut num_of_devices: u32 = 0;
        println!("Acquire webcam devices list. ");
        MFEnumDeviceSources(&imf_attr, &mut devices_opt_ptr, &mut num_of_devices)?;
        
        if num_of_devices == 0 {
            println!("There's no webcam device available. ");
            return Ok(())
        }
        else {
            println!("{} device[s] found. ", num_of_devices);
        }
        
        let mut length: u32 = 0;
        let mut device_name_ptr = PWSTR::null();
        println!("Enumerate webcam devices. ");
        for i in 0..num_of_devices {
            let device_opt: *mut Option<IMFActivate> = devices_opt_ptr.offset(i as isize);
            let device: &IMFActivate = (*device_opt).as_ref().unwrap();
            if let Ok(_) = device.GetAllocatedString(
                &MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME, 
                &mut device_name_ptr, 
                &mut length) {
                
                let device_name = device_name_ptr.to_string()?;
                println!("{}: {}", i, device_name);
            }
            else {
                println!("Failed to allocate string. ");
            }
        }
    }
    Ok(())
}
