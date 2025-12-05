use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, LoadLibraryA, GetProcAddress};

use std::ffi::c_void;

use crate::hooks::iat_hook::hook_iat;
use crate::util::log;

static mut ORIG: Option<unsafe extern "system" fn(HWND, *const i8, *const i8, u32) -> i32> = None;

unsafe extern "system" fn my_MessageBoxA(
    hwnd: HWND,
    text: *const i8,
    caption: *const i8,
    typ: u32,
) -> i32 {
    log("MessageBoxA intercepted via IAT hook");

    let new_text = b"IAT Hook Successful!\0".as_ptr() as *const i8;
    let new_caption = b"dw-sdk-hook\0".as_ptr() as *const i8;

    if let Some(orig) = ORIG {
        return orig(hwnd, new_text, new_caption, typ);
    }
    0
}

pub unsafe fn install() {
    log("Installing IAT MessageBoxA hook");

    let module = GetModuleHandleA(std::ptr::null());
    if module == 0 {
        log("GetModuleHandleA failed");
        return;
    }

    let h_user32 = LoadLibraryA(b"user32.dll\0".as_ptr());
    let orig = GetProcAddress(h_user32, b"MessageBoxA\0".as_ptr());

    ORIG = Some(std::mem::transmute(orig));

    hook_iat(
        module as *mut u8,
        "user32.dll",
        "MessageBoxA",
        my_MessageBoxA as *mut c_void
    );
}
