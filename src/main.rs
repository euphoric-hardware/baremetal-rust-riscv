#![no_main]
#![no_std]

use riscv_rt::entry;
use riscv_rust_baremetal::htif::{htif_fail, HostFile};
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

    // TODO: exit properly
    loop {}

    // Panic!
    // let x = 1 / (x-y*2);

    // unsafe { let src = z as *const (); core::ptr::read_volatile(src) }
    // for c in b"Hello from Rust!".iter() {
    //     unsafe {
    //         *uart = *c as u8;
    //     }
    // }
}

// Needed for crt_riscv_test.S
// #[no_mangle]
// pub extern "C" fn handle_trap() {
//     loop {}
// }
