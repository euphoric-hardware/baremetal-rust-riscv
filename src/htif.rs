#[repr(C)]
struct Magic {
    n: i64,
    arg0: i64,
    arg1: i64,
    arg2: i64,
}

// TODO: maybe goes in their own elf section?
#[no_mangle]
#[allow(non_upper_case_globals)]
static mut tohost: *const () = core::ptr::null_mut();
#[no_mangle]
#[allow(non_upper_case_globals)]
static mut fromhost: i64 = 0;

pub fn htif_syscall() {
    unsafe {
        let s = "hello rust.\n" as *const str;
        let buf: *const Magic = &Magic { n: 64, arg0: 1, arg1: s as *const () as i64, arg2: 13 };
        tohost = buf as *const ();
        while fromhost == 0 { }

        // let s = "hello world\n" as *const str;
        // let buf: *const [i64] = &[64, 1, s as *const () as i64, 13];
        // tohost = buf as *const ();

        // while fromhost == 0 { }
        // core::ptr::wri
        // let magic = Magic{ a: 64, b: 0, c: 0 };
        // let ptr = &magic as *const Magic;
        // tohost = ptr as *mut i64;
    };
}
