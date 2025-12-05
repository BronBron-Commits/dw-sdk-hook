mod util;
mod loader;
mod hooks;

use windows_sys::Win32::Foundation::BOOL;

#[no_mangle]
pub extern "system" fn DllMain(_hinst: usize, reason: u32, _reserved: usize) -> BOOL {
    if reason == 1 {
        unsafe { loader::init(); }
    }
    1
}
