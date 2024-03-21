#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(core_intrinsics, lang_items, rustc_attrs)]

use core::panic::PanicInfo;

extern "C" {
  fn GET32(addr: u32) -> u32;
}

extern "C" {
  fn PUT32(addr: u32, val: u32);
}

extern "C" {
  fn delay_loop(num: u32);
}

#[repr(C)]
pub struct GPIO {
  GPFSEL: [u32; 6],
  res0: u32,
  GPSET: [u32; 2],
  res1: u32,
  GPCLR: [u32; 2],
  res2: u32,
  GPLEV: [u32; 2],
  res3: u32,
  GPEDS: [u32; 2],
  res4: u32,
  GPREN: [u32; 2],
  res5: u32,
  GPFEN: [u32; 2],
  res6: u32,
}

impl GPIO {
  fn set_output(&self, pin: u32) {
    let fsel: usize = (pin / 10) as usize;
    let offset = pin % 10;

    let addr = (&self.GPFSEL[fsel] as *const u32) as u32;

    let mut existing = unsafe { GET32(addr) };

    existing &= (!0b111 << offset);
    existing |= (0b001 << offset);

    unsafe { PUT32(addr, existing); }
  }

  fn set(&self, pin: u32) {
    let set: usize = (pin / 32) as usize;
    let offset = pin % 32;

    let addr = (&self.GPSET[set] as *const u32) as u32;

    unsafe { PUT32(addr, 1 << offset); }
  }

  fn clear(&self, pin: u32) {
    let clr: usize = (pin / 32) as usize;
    let offset = pin % 32;

    let addr = (&self.GPCLR[clr] as *const u32) as u32;

    unsafe { PUT32(addr, 1 << offset); }
  }

  fn read(&self, pin: u32) -> u32 {
    let lev: usize = (pin / 32) as usize;
    let offset = pin % 32;

    let addr = (&self.GPLEV[lev] as *const u32) as u32;
    let levs = unsafe { GET32(addr) };

    (levs >> offset) & 1
  }
}

fn timer_init() {
  let base = 0x2000_B400;

  let mut e = unsafe { GET32(base) };

  e &= !(1 << 9);
  e |= (1 << 9);

  unsafe { PUT32(base, e) };
}

fn timer_delay_usec(cycles: u32) {
  let counter = 0x2000_3004;

  let t_init = unsafe { GET32(counter) };

  while true {
    let t_now = unsafe { GET32(counter) };

    if t_now - t_init > cycles {
      break;
    }
  }
}

#[no_mangle] // don't mangle the name of this function
pub extern fn krust_entry() {
  timer_init();

  // 0x2... becomes a mutable pointer. We dereference to get values there and chuck them in GPIO struct.
  // We then return a ref to that struct
  let gpio: &GPIO = unsafe { & *(0x2020_0000 as *const GPIO) };
  
  gpio.set_output(20);

  loop {
    // Set on
    gpio.set(20);

    // unsafe { delay_loop(1_000_000); }
    timer_delay_usec(1_000_000);
  
    // Set off
    gpio.clear(20);
    
    let mut i = 0;
    for _ in 0..1_000_000 {
      i += 1;
    }

    timer_delay_usec(i);
  }
}

// This function is called on panic.
#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}