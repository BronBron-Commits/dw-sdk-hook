use std::ffi::CStr;

#[repr(C)]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_U1,
}

#[repr(C)]
pub union IMAGE_THUNK_DATA32_U1 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}

#[repr(C)]
pub struct IMAGE_IMPORT_BY_NAME {
    pub Hint: u16,
    pub Name: u8,
}

pub fn log(msg: &str) {
    println!("[dw-sdk-hook] {msg}");
}

pub unsafe fn get_import_name(thunk_addr: *const u8) -> Result<String, ()> {
    let thunk = &*(thunk_addr as *const IMAGE_THUNK_DATA32);

    let addr = unsafe { thunk.u1.AddressOfData };
    if addr == 0 {
        return Err(());
    }

    let name_struct = addr as *const IMAGE_IMPORT_BY_NAME;
    let name_ptr = unsafe { &(*name_struct).Name as *const u8 };

    let c = CStr::from_ptr(name_ptr as *const i8);
    Ok(c.to_string_lossy().to_string())
}
