# riscv-rust-baremetal
Rust example that compiles to baremetal riscv. Supports HTIF to communicate with host and perform proxy syscalls.

## Quick start
Edit the configuration at `.cargo/config.toml`.
```
rustup target add riscv64gc-unknown-none-elf
cargo build
```

To run a specific benchmark (Requires [Spike](https://github.com/riscv-software-src/riscv-isa-sim)):
```
cargo run --bin bin-name
```

You can also run cargo with specific configuration with the `--config` flag. Example:
```
rustup target add riscv32imac-unknown-none-elf
cargo --config 'build.target="riscv32imac-unknown-none-elf"' \
    --config 'target.riscv32imac-unknown-none-elf.linker="riscv32-unknown-elf-ld"' \
    r --bin median --release
```
## TODO
- [ ] sync syscall (https://stackoverflow.com/questions/72369202/are-mutable-static-primitives-actually-unsafe-if-single-threaded)
    - spin crate provides spinlock, or just write our own
- [x] make work in release mode
- [x] exit properly
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

### Benchmarks
- Port benchmarks from riscv tests
- Cycles should be similar?

### Results
| Benchmark                      | Rust mcycle | C mcycle | Godbolt                         |
|--------------------------------|-------------|----------|---------------------------------|
| median                         | 6449        | 4654     | https://godbolt.org/z/8rdThb57a |
| vvadd                          | 4835        | 2411     | https://godbolt.org/z/e1jcPajGW |
| sort                           | 215018      | x        |                                 |
| qsort                          | 308650      | 123208   |                                 |
| multiply                       | 7590        | 26811    | https://godbolt.org/z/6fjjqP6rq |
| multiply (with --unroll-loops) |             | 20986    | https://godbolt.org/z/6fjjqP6rq |


## Resources
HTIF implementations:
- https://github.com/ucb-bar/libgloss-htif
- https://github.com/riscv-software-src/riscv-tests/tree/master/benchmarks/common

Rust:
- Baremetal rust: https://docs.rust-embedded.org/book/intro/index.html
- Riscv rust: https://github.com/rust-embedded/riscv
