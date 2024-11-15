#![no_main]
#![no_std]

mod htif;
mod hw_util;
mod benchmark;

use benchmark::run_median;
use htif::{htif_fail, HostFile};
use hw_util::csr_test;
use core::fmt::Write;

use riscv_rt::entry;


#[entry]
fn main() -> ! {
    let x = 10;
    let y = 5;
    let mut z = x + y;
    z = z + z + 3;
    htif_fail(1);
    // run_median();
    //
    // let num = csr_test();
    //
    // writeln!(HostFile::stdout(), "{:?}", num).unwrap();
    //
    // writeln!(HostFile::stdout(), "{}", z).unwrap();
    //
    //
    // writeln!(HostFile::from_fd(1), "Hello {} {}", x, y).unwrap();
    // writeln!(HostFile::stdout(), "Hello {}", x+y).unwrap();
    //
    // for i in 0..5 {
    //     writeln!(HostFile::from_fd(1), "{}", i).unwrap();
    // }
    // writeln!(HostFile::stdout(), "{:?}", [1, 2, 3, 4, 5]).unwrap();
    // writeln!(HostFile::stdout(), "{:?}", (1, 2, 3)).unwrap();
    loop {}

    // TODO: exit properly

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

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    writeln!(HostFile::stdout(), "{}", _info).unwrap();
    htif_fail(24);
    loop {}
}
