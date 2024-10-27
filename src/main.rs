#![no_main]
#![no_std]

mod htif;

use htif::htif_syscall;


// A cleaner way is to learn rust macro and implement #[entry]: https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.entry.html
#[no_mangle]
pub extern "C" fn _init() {
    let x: i64 = 17;
    let y: i64 = 39;

    let z = x + y;
    htif_syscall();

    // unsafe { let src = z as *const (); core::ptr::read_volatile(src) }
    // for c in b"Hello from Rust!".iter() {
    //     unsafe {
    //         *uart = *c as u8;
    //     }
    // }

    loop{}
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
    let x: i64 = 19;
    let y: i64 = 47;

    let z = x + y;
    loop{}
}
