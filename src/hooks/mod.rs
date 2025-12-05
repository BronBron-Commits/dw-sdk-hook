pub mod iat_hook;
pub mod messagebox;

pub unsafe fn install_all() {
    messagebox::install();
}
