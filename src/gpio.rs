use crate::asm::{
  GET32,
  PUT32,
};

#[allow(non_snake_case)]
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

  pub fn init() -> &'static GPIO {
    let gpio: &GPIO = unsafe { & *(0x2020_0000 as *const GPIO) };
    gpio
  }

  fn set_func(&self, pin: u32, func: u32) {
    let fsel: usize = (pin / 10) as usize;
    let offset = (pin % 10) * 3;

    let addr = (&self.GPFSEL[fsel] as *const u32) as u32;

    let mut existing = unsafe { GET32(addr) };

    existing &= !(0b111 << offset);
    if func == 1 {
        existing |= 0b001 << offset;
    }

    unsafe { PUT32(addr, existing); }
  }

  pub fn set_input(&self, pin: u32) {
    self.set_func(pin, 0);
  }

  pub fn set_output(&self, pin: u32) {
    self.set_func(pin, 1);
  }

  pub fn set(&self, pin: u32) {
    let set: usize = (pin / 32) as usize;
    let offset = pin % 32;

    let addr = (&self.GPSET[set] as *const u32) as u32;

    unsafe { PUT32(addr, 1 << offset); }
  }

  pub fn set_val(&self, pin: u32, val: u32) {
    if val == 1 {
        self.set(pin);
    } else {
        self.clear(pin);
    }
  }

  pub fn clear(&self, pin: u32) {
    let clr: usize = (pin / 32) as usize;
    let offset = pin % 32;

    let addr = (&self.GPCLR[clr] as *const u32) as u32;

    unsafe { PUT32(addr, 1 << offset); }
  }

  pub fn read(&self, pin: u32) -> u32 {
    let lev: usize = (pin / 32) as usize;
    let offset = pin % 32;

    let addr = (&self.GPLEV[lev] as *const u32) as u32;
    let levs = unsafe { GET32(addr) };

    (levs >> offset) & 1
  }
}