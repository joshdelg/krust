#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(core_intrinsics, lang_items, rustc_attrs)]

use core::panic::PanicInfo;

mod asm;
// use asm::{
//   GET32,
//   PUT32,
//   delay_loop
// };

mod gpio;
use gpio::{
  GPIO
};

mod timer;
use timer::{
  SysTimer
};

mod servo;

mod sw_uart;
use sw_uart::{
  SWUART
};

#[no_mangle] // don't mangle the name of this function
pub extern fn krust_entry() {

  // 0x2... becomes a mutable pointer. We dereference to get values there and chuck them in GPIO struct.
  // We then return a ref to that struct
  let gpio = GPIO::init();

  gpio.set_output(20);
  // gpio.set(20);

  // SysTimer::delay_usec(1_000_000);

  // gpio.clear(20);
  // SysTimer::delay_usec(1_000_000);
  
  SysTimer::cycle_init();
  // gpio.set(20);
  // let s = SysTimer::get_cycles();
  // SysTimer::delay_cycles_exact(s, 700 * 1000 * 1000);
  // gpio.clear(20);
  SWUART::init(gpio);
  SWUART::put_str(gpio, "Hello from kRust!\n");


  // gpio.clear(14);
  // SysTimer::delay_usec(1_000_000);
}

// This function is called on panic.
#[no_mangle]
#[panic_handler]
#[lang = "panic"]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #[lang = "panic_nounwind"] // needed by codegen for non-unwinding panics
// #[rustc_nounwind]
// // // #[rustc_const_unstable(feature = "panic_internals", issue = "none")]
// pub fn panic_nounwind(expr: &'static str) -> ! {
//   loop {}
// }