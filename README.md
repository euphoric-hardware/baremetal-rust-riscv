# riscv-rust-baremetal
Rust example that compiles to baremetal riscv. Supports HTIF to communicate with host and perform proxy syscalls.

## Quick start
Edit the configuration at `.cargo/config.toml`.
```
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv64imac-unknown-none-elf
cargo build
```

To run a specific benchmark (Requires [Spike](https://github.com/riscv-software-src/riscv-isa-sim)):
```
cargo run --bin bin-name
```

## TODO
- [ ] sync syscall (https://stackoverflow.com/questions/72369202/are-mutable-static-primitives-actually-unsafe-if-single-threaded)
- [x] make work in release mode
- [ ] exit properly
- [x] core::write
- [ ] trap handling
- [x] panic handling

## Ideas
- Share rust code/interface with fesvr for syscalls
- Would be cool to debug with gdb using openocd: https://docs.rust-embedded.org/book/start/hardware.html
- Better cargo commands (cargo run with spike -d, build options, specify targets...)
- Improve organization, HTIF as its own package?
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
- Riscv rust: https://github.com/rust-embedded/riscv
