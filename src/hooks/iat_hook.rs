use std::ffi::{CStr, c_void};

use windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_READWRITE};

use crate::util::{log, IMAGE_THUNK_DATA32, IMAGE_IMPORT_BY_NAME};

#[repr(C)]
struct IMAGE_DATA_DIRECTORY {
    VirtualAddress: u32,
    Size: u32,
}

#[repr(C)]
struct IMAGE_OPTIONAL_HEADER32 {
    _pad0: [u8; 96],
    DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}

#[repr(C)]
struct IMAGE_FILE_HEADER {
    _pad: [u8; 20],
}

#[repr(C)]
struct IMAGE_NT_HEADERS32 {
    Signature: u32,
    FileHeader: IMAGE_FILE_HEADER,
    OptionalHeader: IMAGE_OPTIONAL_HEADER32,
}

#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub OriginalFirstThunk: u32,
    pub TimeDateStamp: u32,
    pub ForwarderChain: u32,
    pub Name: u32,
    pub FirstThunk: u32,
}

const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;

pub unsafe fn hook_iat(
    module_base: *mut u8,
    imported_dll: &str,
    function: &str,
    new_fn: *mut c_void,
) {
    let dos = &*(module_base as *const IMAGE_DOS_HEADER);
    let nt = &*(module_base.add(dos.e_lfanew as usize) as *const IMAGE_NT_HEADERS32);

    let import_dir = nt.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT].VirtualAddress;

    if import_dir == 0 {
        log("No import directory");
        return;
    }

    let mut desc = module_base.add(import_dir as usize) as *mut IMAGE_IMPORT_DESCRIPTOR;

    while (*desc).Name != 0 {
        let name_ptr = module_base.add((*desc).Name as usize) as *const i8;
        let dll = CStr::from_ptr(name_ptr).to_string_lossy().to_lowercase();

        if dll == imported_dll.to_lowercase() {
            log("Found DLL import descriptor");

            let thunk = module_base.add((*desc).FirstThunk as usize) as *mut *mut c_void;
            let mut i = 0;

            loop {
                let entry = thunk.add(i);
                if (*entry).is_null() {
                    break;
                }

                if let Ok(name) = crate::util::get_import_name(*entry as *const u8) {
                    if name == function {
                        log("Patching IAT entry");

                        let mut old = 0u32;
                        VirtualProtect(entry as *mut _, 4, PAGE_READWRITE, &mut old);

                        *entry = new_fn;

                        VirtualProtect(entry as *mut _, 4, old, &mut old);
                        return;
                    }
                }

                i += 1;
            }
        }

        desc = desc.add(1);
    }

    log("IAT symbol not found");
}
