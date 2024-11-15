use std::{env, error::Error};
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // assemble the `asm.s` file
    println!("cargo::warning={}", env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap());
    // assert!(false, "{}", env::var("CARGO_CFG_TARGET_").unwrap());
    // Build::new()
    //     .file("src/crt.S")
    //     .flag(march)
    //     .flag(mabi)
    //     .compile("asm");

    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=memory.x");

    Ok(())
}
