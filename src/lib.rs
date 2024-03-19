#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(core_intrinsics, lang_items)]

use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern fn krust_entry() {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // loop {}

    const FSEL: *mut u32 = 0x20200008 as *mut u32;
    const SET0: *mut u32 = 0x2020001C as *mut u32;
    // // const CLR0: *mut u32 = 0x20200028 as *mut u32;

    unsafe {
        let mut existing = volatile_load(FSEL);
        existing &= (!0b111);
        existing |= 0b001;
        volatile_store(FSEL, existing);

        // Set on
        // SET0.write_volatile(1 << 20);
        volatile_store(SET0, 1 << 20);
    }
}

/*
void gpio_set_output(unsigned pin)
{
  if (pin >= 32)
    return;

  // Calculate offset from base to see which 32-bit sequence, then offset within 32-bits
  unsigned gpfsel_num = pin / 10;
  unsigned fsel_num = (pin % 10) * 3;

  unsigned *GPFSEL_ADDR = (unsigned *)GPIO_BASE;

  unsigned existing = get32(&GPFSEL_ADDR[gpfsel_num]);
  unsigned mask = 0b111 << fsel_num;

  existing &= ~mask;
  existing |= (GPIO_FUNC_OUTPUT << fsel_num);

  put32(&GPFSEL_ADDR[gpfsel_num], existing);
}
*/

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}