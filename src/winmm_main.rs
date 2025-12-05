use std::fs::OpenOptions;
use std::io::Write;

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("C:\\dw_winmm_log.txt")
    {
        let _ = writeln!(f, "{}", msg);
    }
}

pub fn initialize() {
    log("winmm.dll initialized");
}

pub fn shutdown() {
    log("winmm.dll shutting down");
}
