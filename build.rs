use std::env;

fn main() {
    // hacky but works
    let target_dir = format!("{}/../../..", env::var("OUT_DIR").unwrap());

    #[cfg(windows)]
    {
        println!("cargo::rustc-cdylib-link-arg=/DEF:ttlego-proxy/proxy.def");
        println!("cargo::rustc-cdylib-link-arg=/OUT:{target_dir}/XInput1_3.dll");
    }
}
