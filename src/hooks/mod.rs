mod messagebox;
pub mod inline_hook;

pub fn install_all() {
    crate::util::log("install_all() called");

    unsafe {
        messagebox::install();
    }
}
