use std::{env, error::Error};
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // assemble the `asm.s` file
    println!("cargo::warning={}", env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap());
    // assert!(false, "{}", env::var("CARGO_CFG_TARGET_").unwrap());
    let mut march = "-march=rv64imaczicsr";
    let mut mabi = "-mabi=lp64";
    if 32 == env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().parse().unwrap() {
        march = "-march=rv32imaczicsr";
        mabi = "-mabi=ilp32";
    }
    Build::new()
        .file("src/crt.S")
        .flag(march)
        .flag(mabi)
        .compile("asm");

    println!("cargo::rerun-if-changed=src");

    Ok(())
}
