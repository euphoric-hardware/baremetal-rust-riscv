use core::arch::asm;


pub fn csr_test() -> usize {
    let mut x: usize = 10;
    unsafe {
        asm!(
            // the "mcycle" csr
            "csrr {x}, 0xb00",
            x = inout(reg)x,
        );
    }
    return x
}
