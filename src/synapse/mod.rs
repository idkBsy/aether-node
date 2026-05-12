// SUMMARY: The Synapse Bridge providing safe Rust FFI wrappers over C packet ingestion.
use crate::error::AetherError;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct RawSignal {
    pub cycle_count: u64,
    pub caplen: u32,
    pub len: u32,
    pub data: *const u8,
}

pub type RawSignalCallback = extern "C" fn(*const RawSignal);

extern "C" {
    #[allow(dead_code)]
    pub fn get_rdtsc_raw() -> u64;
    fn init_capture(device: *const c_char, callback: RawSignalCallback) -> i32;
    fn start_capture_loop();
    #[allow(dead_code)]
    fn stop_capture();
}

pub fn initialize_capture(device: &str, callback: RawSignalCallback) -> Result<(), AetherError> {
    let dev_cstr = CString::new(device)
        .map_err(|_| AetherError::CaptureInitFault("Invalid device name".to_string()))?;
    
    let res = unsafe { init_capture(dev_cstr.as_ptr(), callback) };
    if res == 0 {
        Ok(())
    } else {
        Err(AetherError::CaptureInitFault("Failed to initialize libpcap".to_string()))
    }
}

pub fn start() {
    unsafe { start_capture_loop() }
}

#[allow(dead_code)]
pub fn stop() {
    unsafe { stop_capture() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdtsc_timing_test() {
        let t1 = unsafe { get_rdtsc_raw() };
        let t2 = unsafe { get_rdtsc_raw() };
        assert!(t2 >= t1, "RDTSC output must increment");
    }
}
