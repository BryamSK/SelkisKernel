#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::BaseRevision;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static BASE_REVISION: BaseRevision = BaseRevision::new();

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    if !BASE_REVISION.is_supported() {
        loop {}
    }

    loop {}
}