use std::env;

#[cfg(feature = "export")]
use std::path::PathBuf;

fn main() {
    match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("macos") | Ok("ios") => println!("cargo:rustc-link-lib=framework=IOKit"),
        _ => {}
    }

    #[cfg(feature = "export")]
    if env::var_os("_CBINDGEN_IS_RUNNING").is_none() {
        let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"));
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set"));

        println!("cargo:rerun-if-changed=src");
        println!("cargo:rerun-if-changed=cbindgen.toml");
        println!("cargo:rerun-if-changed=../../co3/src");

        cbindgen::Builder::new()
            .with_crate(&crate_dir)
            .with_config(cbindgen::Config::from_file(crate_dir.join("cbindgen.toml")).expect("read cbindgen.toml"))
            .generate()
            .expect("generate the C API header")
            .write_to_file(out_dir.join("battery_ffi.h"));
    }
}
