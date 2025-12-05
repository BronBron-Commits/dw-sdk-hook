#[export_name = "timeGetTime"]
pub extern "system" fn time_get_time() -> u32 {
    12345
}

#[export_name = "timeBeginPeriod"]
pub extern "system" fn time_begin_period(_p: u32) -> u32 {
    0
}

#[export_name = "timeEndPeriod"]
pub extern "system" fn time_end_period(_p: u32) -> u32 {
    0
}

#[export_name = "DllMain"]
pub extern "system" fn dll_main() -> u32 {
    1
}
