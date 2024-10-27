# riscv-rust-baremetal
Rust example that compiles to baremetal riscv. Supports HTIF to communicate with host and perform proxy syscalls.

## Quick start
```
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv64imac-unknown-none-elf
cargo run
```
Requires [Spike](https://github.com/riscv-software-src/riscv-isa-sim).

## TODO
- Use libgloss to figure out how HTIF works and write documentation for it.
- exit properly
- core::write
- trap handling
- panic handling

## Plans
- Would be cool to debug with gdb using openocd: https://docs.rust-embedded.org/book/start/hardware.html
- Better cargo commands (cargo run with spike -d, build options, specify targets...)
- Improve organization, HTIF as its own crate?
- HAL / board config? IDK how hardware works
- Other examples use memory.x and include it in linker script to sepecify memory layout, could be useful/cleaner.
- What should be the init assembly? Copy from other projects? minimal? HW dependent?
    - Not sure what zeroing registers and other magic does

## Resources
HTIF implementations:
- https://github.com/ucb-bar/libgloss-htif
- https://github.com/riscv-software-src/riscv-tests/tree/master/benchmarks/common

Rust:
- Baremetal rust: https://docs.rust-embedded.org/book/intro/index.html

Similar work: https://github.com/riscv-software-src/riscv-isa-sim
