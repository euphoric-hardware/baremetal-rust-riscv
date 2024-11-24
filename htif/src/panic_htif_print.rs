use core::panic::PanicInfo;
use crate::{HostFile, htif_fail};
use core::fmt::Write;
pub use core::writeln;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    writeln!(HostFile::stdout(), "{}", _info).unwrap();
    htif_fail(24);
    loop {}
}
