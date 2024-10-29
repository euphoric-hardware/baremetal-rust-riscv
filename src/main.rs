#![no_main]
#![no_std]

mod htif;

use htif::{htif_fail, HostFile};
use core::fmt::Write;


// A cleaner way is to learn rust macro and implement #[entry]: https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.entry.html
#[no_mangle]
pub extern "C" fn _init() {
    main();

    loop{}
}

fn main() {
    let x = 10;
    let y = 5;
    writeln!(HostFile::from_fd(1), "Hello {} {}", x, y).unwrap();
    writeln!(HostFile::stdout(), "Hello {}", x+y).unwrap();

    for i in 0..5 {
        writeln!(HostFile::from_fd(1), "{}", i).unwrap();
    }
    writeln!(HostFile::stdout(), "{:?}", [1, 2, 3, 4, 5]).unwrap();
    writeln!(HostFile::stdout(), "{:?}", (1, 2, 3)).unwrap();

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
