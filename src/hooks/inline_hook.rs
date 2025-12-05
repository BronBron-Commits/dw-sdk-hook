use region::{protect, Protection};
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
};
use std::ptr;
use std::ffi::c_void;

const JMP_OPCODE: u8 = 0xE9;

pub struct InlineHook {
    pub target: *mut u8,
    pub trampoline: *mut u8,
    pub bytes_stolen: usize,
}

impl InlineHook {
    pub unsafe fn new(target: *mut u8, detour: *mut u8) -> Self {
        let bytes_stolen = 5;

        // Allocate trampoline using Windows executable memory
        let tramp = VirtualAlloc(
            std::ptr::null_mut(),
            (bytes_stolen + 5) as usize,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        ) as *mut u8;

        // Copy original bytes
        ptr::copy_nonoverlapping(target, tramp, bytes_stolen);

        // Build jump from tramp → target+5
        *tramp.add(bytes_stolen) = JMP_OPCODE;

        // rel32 = dest - (src + 5)
        let return_addr = target.add(bytes_stolen);
        let rel_back = (return_addr as isize - tramp.add(bytes_stolen + 1) as isize) as i32;

        ptr::copy_nonoverlapping(
            &rel_back as *const i32 as *const u8,
            tramp.add(bytes_stolen + 1),
            4,
        );

        // Make target writable
        protect(target, bytes_stolen, Protection::READ_WRITE_EXECUTE).unwrap();

        // Write JMP opcode to target
        *target = JMP_OPCODE;

        // Calculate detour jump
        let rel_detour = (detour as isize - target.add(1) as isize) as i32;

        ptr::copy_nonoverlapping(
            &rel_detour as *const i32 as *const u8,
            target.add(1),
            4,
        );

        InlineHook {
            target,
            trampoline: tramp,
            bytes_stolen,
        }
    }

    pub unsafe fn trampoline(&self) -> *const c_void {
        self.trampoline as *const c_void
    }
}
