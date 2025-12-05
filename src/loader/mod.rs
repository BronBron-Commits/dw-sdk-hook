pub fn initialize() {
    crate::util::log("dw-sdk-hook loaded into process");
    crate::hooks::install_all();
}

pub fn shutdown() {
    crate::util::log("dw-sdk-hook shutting down");
}
