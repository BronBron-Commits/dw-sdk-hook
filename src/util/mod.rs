use std::fs::OpenOptions;
use std::io::Write;

// You may change this path after testing
static LOG_PATH: &str = "C:\\\\dw_sdk_hook_log.txt";

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
    {
        let _ = writeln!(f, "{}", msg);
    }
}
