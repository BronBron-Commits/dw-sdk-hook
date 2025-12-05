use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_OK,
};

use std::ffi::c_char;
use std::ptr;

use crate::hooks::inline_hook::InlineHook;

static mut ORIGINAL_MESSAGEBOX: Option<InlineHook> = None;

extern "system" fn my_messagebox_a(
    hwnd: *mut c_char,
    text: *mut c_char,
    caption: *mut c_char,
    msg_type: u32,
) -> i32 {
    crate::util::log("MessageBoxA intercepted via inline hook");

    // call original via trampoline
    unsafe {
        let tramp = ORIGINAL_MESSAGEBOX.as_ref().unwrap().trampoline();

        let func: extern "system" fn(
            *mut c_char,
            *mut c_char,
            *mut c_char,
            u32
        ) -> i32 = std::mem::transmute(tramp);

        func(
            hwnd,
            b"Inline Hook Successful!\0".as_ptr() as *mut c_char,
            b"dw-sdk-hook\0".as_ptr() as *mut c_char,
            MB_OK,
        )
    }
}

pub unsafe fn install() {
    crate::util::log("Installing inline MessageBoxA hook...");

    let target = MessageBoxA as usize as *mut u8;
    let detour = my_messagebox_a as usize as *mut u8;

    let hook = InlineHook::new(target, detour);

    ORIGINAL_MESSAGEBOX = Some(hook);

    crate::util::log("MessageBoxA inline hook installed");
}
