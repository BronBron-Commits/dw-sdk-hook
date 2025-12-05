#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod loader;
mod hooks;
mod util;

use crate::util::log;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::SystemServices::*;
use windows_sys::Win32::System::LibraryLoader::{LoadLibraryA, DisableThreadLibraryCalls};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS
};

fn get_process_name() -> Option<String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        let mut entry: PROCESSENTRY32 = core::mem::zeroed();
        entry.dwSize = core::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 0 {
            return None;
        }

        loop {
            let name_c = entry.szExeFile.iter().take_while(|&&c| c != 0).cloned().collect::<Vec<_>>();
            let name = String::from_utf8_lossy(&name_c).to_string();

            if name.to_lowercase().contains("dworld.exe") {
                return Some(name);
            }

            if Process32Next(snapshot, &mut entry) == 0 {
                break;
            }
        }
    }
    None
}

unsafe fn try_load_sdk_hook() {
    if get_process_name().is_some() {
        let dll_name = b"dw-sdk-hook.dll\0";

        let result = LoadLibraryA(dll_name.as_ptr());
        if result != 0 {
            log("dw-sdk-hook.dll successfully loaded by winmm.dll");
        } else {
            log("FAILED to load dw-sdk-hook.dll");
        }
    }
}

#[no_mangle]
extern "system" fn DllMain(hinst: HINSTANCE, reason: u32, reserved: *mut core::ffi::c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe { DisableThreadLibraryCalls(hinst); }
            unsafe { try_load_sdk_hook(); }
            loader::initialize();
        }
        DLL_PROCESS_DETACH => {
            loader::shutdown();
        }
        _ => {}
    }
    TRUE
}
