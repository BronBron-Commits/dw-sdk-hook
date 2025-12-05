#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod loader;
mod hooks;
mod util;

use windows_sys::Win32::Foundation::{BOOL, HINSTANCE, TRUE};
use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows_sys::Win32::System::LibraryLoader::DisableThreadLibraryCalls;

#[no_mangle]
extern "system" fn DllMain(hinst: HINSTANCE, reason: u32, _reserved: *mut core::ffi::c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe { DisableThreadLibraryCalls(hinst); }
            loader::initialize();
        }
        DLL_PROCESS_DETACH => {
            loader::shutdown();
        }
        _ => {}
    }

    TRUE
}
