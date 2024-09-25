pub mod waveforms_i2c{
    use std::ffi::CStr;
    use libloading::{Library, Symbol};
    use std::os::raw::{c_char, c_double, c_int, c_uchar};

    type FDwfDeviceOpenFn = unsafe extern "C" fn(i32, *mut i32) -> i32;
    type FDwfGetLastErrorMsgFn = unsafe extern "C" fn(*mut c_char);
    type FDwfDigitalI2cRateSetFn = unsafe extern "C" fn(i32, c_double) -> i32;
    type FDwfDigitalI2cSclSetFn = unsafe extern "C" fn(i32, c_int) -> i32;
    type FDwfDigitalI2cSdaSetFn = unsafe extern "C" fn(i32, c_int) -> i32;
    type FDwfDigitalI2cSpyStartFn = unsafe extern "C" fn(i32) -> i32;
    type FDwfDigitalI2cSpyStatusFn = unsafe extern "C" fn(i32, *mut c_int, *mut c_int, *mut c_uchar, *mut c_int, *mut c_int) -> i32;
    type FDwfDeviceCloseAllFn = unsafe extern "C" fn();

    pub struct WaveformsI2cControl{
        lib: Library,
        hdwf: i32,
    }

    impl WaveformsI2cControl{

        pub fn fdwf_get_last_error_msg(&self) -> Symbol<FDwfGetLastErrorMsgFn> {
            unsafe { self.lib.get(b"FDwfGetLastErrorMsg\0").expect("could not find function FDwfGetLastErrorMsg in lib") }
        }
        pub fn fdwf_device_open(&self) -> Symbol<FDwfDeviceOpenFn> {
            unsafe { self.lib.get(b"FDwfDeviceOpen\0").expect("could not find function FDwfDeviceOpen in lib") }
        }
        pub fn fdwf_digital_i2c_rate_set(&self) -> Symbol<FDwfDigitalI2cRateSetFn> {
            unsafe { self.lib.get(b"FDwfDigitalI2cRateSet\0").expect("could not find function FDwfDigitalI2cRateSet in lib") }
        }
        pub fn fdwf_digital_i2c_scl_set(&self) -> Symbol<FDwfDigitalI2cSclSetFn> {
            unsafe { self.lib.get(b"FDwfDigitalI2cSclSet\0").expect("could not find function FDwfDigitalI2cSclSet in lib") }
        }
        pub fn fdwf_digital_i2c_sda_set(&self) -> Symbol<FDwfDigitalI2cSdaSetFn> {
            unsafe { self.lib.get(b"FDwfDigitalI2cSdaSet\0").expect("could not find function FDwfDigitalI2cSdaSet in lib") }
        }
        pub fn fdwf_digital_i2c_spy_start(&self) -> Symbol<FDwfDigitalI2cSpyStartFn> {
            unsafe { self.lib.get(b"FDwfDigitalI2cSpyStart\0").expect("could not find function FDwfDigitalI2cSpyStart in lib") }
        }
        pub fn fdwf_digital_i2c_spy_status(&self) -> Symbol<FDwfDigitalI2cSpyStatusFn> {
            unsafe { self.lib.get(b"FDwfDigitalI2cSpyStatus\0").expect("could not find function FDwfDigitalI2cSpyStatus in lib") }
        }
        pub fn fdwf_device_close_all(&self) ->  Symbol<FDwfDeviceCloseAllFn> {
            unsafe { self.lib.get(b"FDwfDeviceCloseAll\0").expect("could not find function FDwfDeviceCloseAll in lib") }
        }

        pub fn new(baud_rate: u32, scl_pin: u8, sda_pin: u8) -> Result<WaveformsI2cControl, String> {
            WaveformsI2cControl::initialise(WaveformsI2cControl {
                lib: unsafe { Library::new("dwf.dll").map_err(|e| e.to_string()) }?,
                hdwf: 0,
            }, baud_rate, scl_pin, sda_pin)
        }

        fn initialise(mut self, baud_rate: u32, scl_pin: u8, sda_pin: u8) -> Result<Self, String> {
            unsafe {
                let mut hdwf = 0;
                println!("waveforms_i2c initialising");

                let dev_open = self.fdwf_device_open();
                let fdwf_digital_i2c_rate_set = self.fdwf_digital_i2c_rate_set();
                let fdwf_digital_i2c_scl_set = self.fdwf_digital_i2c_scl_set();
                let fdwf_digital_i2c_sda_set = self.fdwf_digital_i2c_sda_set();
                let fdwf_digital_i2c_spy_start = self.fdwf_digital_i2c_spy_start();
                let fdwf_get_last_error_msg = self.fdwf_get_last_error_msg();

                println!("Opening device");
                if dev_open(-1, &mut hdwf) == 0 {
                    eprintln!("Failed to open device");
                    let mut err_msg_buffer: [c_char; 512] = [0; 512];
                    fdwf_get_last_error_msg(err_msg_buffer.as_mut_ptr());
                    let err_msg = CStr::from_ptr(err_msg_buffer.as_ptr()).to_string_lossy();
                    eprintln!("Error: {}", err_msg);
                    return Err(err_msg.to_string());
                }

                if fdwf_digital_i2c_rate_set(hdwf, baud_rate.into()) == 0 {
                    return Err("Failed to set I2C rate".to_string());
                }

                if fdwf_digital_i2c_scl_set(hdwf, scl_pin.into()) == 0 {
                    return Err("Failed to set SCL".to_string());
                }

                if fdwf_digital_i2c_sda_set(hdwf, sda_pin.into()) == 0 {
                    return Err("Failed to set SDA".to_string());
                }

                // Start I2C Spy
                if fdwf_digital_i2c_spy_start(hdwf) == 0 {
                    return Err("Failed to start I2C spy".to_string());
                }
                else {
                    self.hdwf = hdwf;
                    Ok(self)
                }
            }

        }
        

        pub fn close(&self) -> Result<(), String> {
            unsafe {
                let fdwf_device_close_all = self.fdwf_device_close_all();
                fdwf_device_close_all();
            }
            Ok(())
        }

        pub fn read(&self) -> Result<String, String> {
            let n_data: i32 = 16;
            let mut f_start: c_int = 0;
            let mut f_stop: c_int = 0;
            let mut rg_data: [c_uchar; 16] = [0; 16];
            let mut c_data: c_int;
            let mut i_nak: c_int = 0;
            let mut msg = Vec::new();
            
            let fdwf_digital_i2c_spy_status = self.fdwf_digital_i2c_spy_status();
            let fdwf_get_last_error_msg = self.fdwf_get_last_error_msg();

            unsafe {
                c_data = n_data;
                if fdwf_digital_i2c_spy_status(
                    self.hdwf,
                    &mut f_start,
                    &mut f_stop,
                    rg_data.as_mut_ptr(),
                    &mut c_data,
                    &mut i_nak,
                ) == 0
                {
                    eprintln!("Communication with the device failed.");
                    let mut err_msg_buffer: [c_char; 512] = [0; 512];
                    fdwf_get_last_error_msg(err_msg_buffer.as_mut_ptr());
                    let err_msg = CStr::from_ptr(err_msg_buffer.as_ptr()).to_string_lossy();
                    eprintln!("Error: {}", err_msg);
                    return Err(err_msg.to_string());
                }

                let mut msg = Vec::new();

                // Start condition handling
                if f_start == 1 {
                    msg.push("Start".to_string());
                }
                else if f_start == 2 {
                    msg.push("ReStart".to_string());
                }
            }

            // Data handling
            for i in 0..c_data {
                if i == 0 && f_start != 0 {
                    msg.push(format!("{:#x}", rg_data[i as usize] >> 1));
                    if rg_data[i as usize] & 1 != 0 {
                        msg.push("RD".to_string());
                    }
                    else {
                        msg.push("WR".to_string());
                    }
                } else {
                    msg.push(format!("{:#x}", rg_data[i as usize]));
                }
            }

            // Stop condition
            if f_stop != 0 {
                msg.push("Stop".to_string());
            }

            // NAK or error handling
            if i_nak > 0 {
                msg.push(format!("NAK: {}", i_nak));
            }
            else if i_nak < 0 {
                //msg.push(format!("Error: {}", i_nak));
            }

            if !msg.is_empty() {
                return Ok(msg.join(" "));
            }
            else {
                return Err("No data received".to_string());
            }
        }
    }
        

}