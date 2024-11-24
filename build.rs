use std::{env, error::Error};
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // assemble the `asm.s` file
    println!("cargo::rerun-if-changed=memory.x");

    Ok(())
}
