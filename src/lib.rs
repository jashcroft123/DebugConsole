extern "system" {
    fn AllocConsole() -> i32;
    fn GetStdHandle(handle: u32) -> *mut std::ffi::c_void;
    fn WriteConsoleA(
        handle: *mut std::ffi::c_void,
        buffer: *const u8,
        len: u32,
        written: *mut u32,
        reserved: *mut std::ffi::c_void,
    ) -> i32;
}

const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

#[no_mangle]
pub extern "C" fn OpenConsole() {
    unsafe {
        AllocConsole();
    }
}

#[no_mangle]
pub extern "C" fn DebugPrint(msg: *const c_char) {
    println!("DebugPrint called");
    unsafe {
        if msg.is_null() {
            return;
        }
        let s = CStr::from_ptr(msg).to_string_lossy();
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut written = 0u32;
        WriteConsoleA(
            h,
            s.as_ptr() as *const u8,
            s.len() as u32,
            &mut written,
            ptr::null_mut(),
        );
        WriteConsoleA(
            h,
            b"\n".as_ptr(),
            1,
            &mut written,
            ptr::null_mut(),
        );
    }
}

#[no_mangle]
pub extern "C" fn PrintSelfTest() {
        DebugPrint(b"Console Debug Self Test Successful!\0".as_ptr() as *const c_char);
}