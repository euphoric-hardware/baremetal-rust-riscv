#![no_main]
#![no_std]

use riscv_rt::entry;
use htif::{exit, HostFile};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let x = 10;
    let y = 5;
    let mut z = x + y;
    z = z + z + 3;

    writeln!(HostFile::stdout(), "{}", z).unwrap();


    writeln!(HostFile::from_fd(1), "Hello {} {}", x, y).unwrap();
    writeln!(HostFile::stdout(), "Hello {}", x+y).unwrap();

    for i in 0..5 {
        writeln!(HostFile::from_fd(1), "{}", i).unwrap();
    }
    writeln!(HostFile::stdout(), "{:?}", [1, 2, 3, 4, 5]).unwrap();
    writeln!(HostFile::stdout(), "{:?}", (1, 2, 3)).unwrap();

    exit(0);
    // Panic!
    // let x = 1 / (x-y*2);
}
