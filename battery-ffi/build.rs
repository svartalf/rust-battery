#[cfg(feature = "cbindgen")]
fn build_header() {
    use std::env;
    use std::path::PathBuf;

    let crate_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not defined");
    let out_dir = PathBuf::from(env::var("OUT_DIR")
        .expect("OUT_DIR env var is not defined"));

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(out_dir.join("battery_ffi.h"));
}

fn main() {
    #[cfg(feature = "cbindgen")]
    build_header()
}
