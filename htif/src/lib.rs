#![no_std]

pub mod panic_htif_print;


use core::ptr::{read_volatile, write_volatile};
use core::fmt;

#[no_mangle]
#[allow(non_upper_case_globals)]
#[link_section = ".htif"]
static mut tohost: *const () = core::ptr::null_mut();
#[no_mangle]
#[allow(non_upper_case_globals)]
#[link_section = ".htif"]
static mut fromhost: i64 = 0;

pub struct HostFile {
    fd: u64,
}

impl HostFile {
    pub fn from_fd(fd: u64) -> HostFile {
        HostFile{ fd }
    }

    pub fn stdout() -> HostFile {
        HostFile::from_fd(1)
    }
}

impl fmt::Write for HostFile {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(self.fd, s);
        Ok(())
    }
}

// TODO: maybe this whole function should be unsafe? Depends on spec of HTIF/invalid syscalls
pub fn htif_syscall(n: u64, arg0: u64, arg1: u64, arg2: u64) {
    unsafe {
        // Array layouts explained here, seems to be the same as layout in C:
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
        // struct with repr(C) would also work

        // Create an empty array, then use write_volatile to make sure values are written on the
        // stack.
        let mut buf: [u64; 8] = [0; 8];
        write_volatile(&mut buf, [n, arg0, arg1, arg2, 0, 0, 0, 0]);
        let ptr: *const [u64] = &buf;

        write_volatile(&raw mut tohost, ptr as *const ());
        while read_volatile(&raw const fromhost) == 0 { }
        write_volatile(&raw mut fromhost, 0);
    }
}

pub fn htif_fail(n: i64) {
    unsafe {
        write_volatile(&raw mut tohost, (n*2 + 1) as *const ());
        while read_volatile(&raw const fromhost) == 0 { }
        write_volatile(&raw mut fromhost, 0);
    }
}

pub fn write(fd: u64, s: &str) {
    // let hello = "hello rust.\n" ;
    let len = s.len();
    let s = s as *const str;
    htif_syscall(64, fd, s as *const () as u64, len.try_into().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
