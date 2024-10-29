use core::ptr::write_volatile;

// TODO: maybe goes in their own elf section (use link_section attribute)?
#[no_mangle]
#[allow(non_upper_case_globals)]
static mut tohost: *const () = core::ptr::null_mut();
#[no_mangle]
#[allow(non_upper_case_globals)]
static mut fromhost: i64 = 0;

// TODO: maybe this whole function should be unsafe? Depends on spec of HTIF/invalid syscalls
pub fn htif_syscall(n: u64, arg0: u64, arg1: u64, arg2: u64) {
    unsafe {
        // Array layouts explained here, seems to be the same as layout in C:
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
        // struct with repr(C) would also work

        // Create an empty array, then use write_volatile to make sure values are written on the
        // stack.
        let mut buf: [u64; 4] = [0; 4];
        write_volatile(&mut buf, [n, arg0, arg1, arg2]);
        let ptr: *const [u64] = &buf;

        write_volatile(&raw mut tohost, ptr as *const ());
        while fromhost == 0 { }
    }
}

pub fn htif_fail(n: i64) {
    unsafe {
        write_volatile(&raw mut tohost, (n*2 + 1) as *const ());
        while fromhost == 0 { }
    }
}

pub fn print() {
    let hello = "hello rust.\n" ;
    let s = hello as *const str;
    htif_syscall(64, 1, s as *const () as u64, 13);
}
