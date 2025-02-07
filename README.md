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
    - https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
- [x] make work in release mode
- [x] exit properly
- [x] core::write
- [ ] trap handling
- [x] panic handling
- [ ] double check qsort

## Ideas
- Unsafe rust (get_unchecked, unwrap_unchecked)
- Idiomatic rust using iterators
- Share rust code/interface with fesvr for syscalls
- Would be cool to debug with gdb using openocd: https://docs.rust-embedded.org/book/start/hardware.html

### Benchmarks
- Port benchmarks from riscv tests
- Cycles should be similar?
- Not sure if cycles are accurate, might need more testing / investigating.
- Figured out why mcycle is worst: no vector instructions

- Tower: The the original riscv-tests uses linked list. Linked list normally sucks, but maybe in embedded programming it is good. Still need to decide how to port
- memcpy: clone (memcpy) uses 5x more mcycle, maybe look at rust documentation. C version unrolls the loop
- radix sort: https://crates.io/crates/rdx/0.0.7

#### Conclusion
Hard to recreate some of the tests without using unsafe and pointers, but then we would just be writing C. Maybe we should still do that.

Borrow checker is pretty good. Safety gaurantees are nice too. For example, global static is unsafe by default, so the programmer knows that they need sync or redesign (for porting riscv-tests, I make the caller track data for now instead of global statics).

### Results
| Benchmark                        | Rust mcycle | C mcycle | Godbolt                         |
|----------------------------------|-------------|----------|---------------------------------|
| median                           | 6449        | 4654     | https://godbolt.org/z/8rdThb57a |
| sort                             | 215018      | x        |                                 |
| qsort                            | 231015      | 123208   |                                 |
| qsort (with no bounds checking)  | 184832      |          |                                 |
| rsort                            | TODO        | 171156   |                                 |
| towers                           | TODO        |          |                                 |
| vvadd                            | 4835        | 2411     | https://godbolt.org/z/e1jcPajGW |
| memcpy                           | 9540        | 5515     | https://godbolt.org/z/Wz3T6oP96 |
| multiply                         | 7590        | 26811    | https://godbolt.org/z/6fjjqP6rq |
| multiply (with --unroll-loops)   |             | 20986    | https://godbolt.org/z/6fjjqP6rq |


## Resources
HTIF implementations:
- https://github.com/ucb-bar/libgloss-htif
- https://github.com/riscv-software-src/riscv-tests/tree/master/benchmarks/common

Rust:
- Baremetal rust: https://docs.rust-embedded.org/book/intro/index.html
- Riscv rust: https://github.com/rust-embedded/riscv
