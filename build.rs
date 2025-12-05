fn main() {
    // Correct way: use -DEF: instead of /DEF:
    println!("cargo:rustc-link-arg=-DEF:winmm.def");
}
