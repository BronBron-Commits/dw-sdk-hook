use crate::util::log;

pub fn init() {
    log("dw-sdk-hook loader init start");

    unsafe {
        crate::hooks::install_all();
    }

    log("dw-sdk-hook loader init done");
}
