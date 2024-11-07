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
- [ ] sync syscall (https://stackoverflow.com/questions/72369202/are-mutable-static-primitives-actually-unsafe-if-single-threaded)
- [x] make work in release mode
- [ ] exit properly
- [x] core::write
- [ ] trap handling
- [ ] panic handling

## Plans
- Share rust code/interface with fesvr for syscalls
- Would be cool to debug with gdb using openocd: https://docs.rust-embedded.org/book/start/hardware.html
- Better cargo commands (cargo run with spike -d, build options, specify targets...)
- Improve organization, HTIF as its own crate?
- HAL / board config? IDK how hardware works
- What should be the init assembly and linker scripts? Copy from other projects? minimal? HW dependent?
    - Not sure what zeroing registers and other magic does
- Use #[entry]

### Benchmarks
- Port benchmarks from riscv tests
- Cycles should be similar?

## Resources
HTIF implementations:
- https://github.com/ucb-bar/libgloss-htif
- https://github.com/riscv-software-src/riscv-tests/tree/master/benchmarks/common

Rust:
- Baremetal rust: https://docs.rust-embedded.org/book/intro/index.html

Similar work: https://github.com/search?q=riscv+baremetal+lang%3Arust&type=repositories
