use std::env;

fn main() {
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = format!("{}/../../..", env::var("OUT_DIR").unwrap()); // hacky but works

    match std::env::var("CARGO_CFG_TARGET_FAMILY") {
        Ok(family) => {
            if family.contains("windows") {
                println!("cargo::rustc-cdylib-link-arg=/DEF:{lib_dir}/proxy.def");
                println!("cargo::rustc-cdylib-link-arg=/OUT:{target_dir}/XInput1_3.dll");
            }
        }
        _ => {}
    }
}
