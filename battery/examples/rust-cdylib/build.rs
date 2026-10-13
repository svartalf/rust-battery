use std::env;

fn main() {
    let lib_dir = env::var("BATTERY_LIB_DIR")
        .expect("set BATTERY_LIB_DIR to the directory containing libbattery");
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=battery");
}
