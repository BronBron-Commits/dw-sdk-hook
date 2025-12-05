use std::ffi::c_void;

#[no_mangle]
pub extern "system" fn timeGetTime() -> u32 {
    // Return a fake tick count. This can be incremented if needed.
    42
}

#[no_mangle]
pub extern "system" fn DllMain(_: *mut c_void, _: u32, _: *mut c_void) -> i32 {
    1 // TRUE
}
