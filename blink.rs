#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { DDRB.write_volatile(DDRB.read_volatile() | (1 << 5));}
    loop {
        unsafe {PORTB.write_volatile(PORTB.read_volatile() | (1 << 5)); }
        delay(1000000);
        unsafe {PORTB.write_volatile(PORTB.read_volatile() & !(1 << 5));}
        delay(100000);
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
