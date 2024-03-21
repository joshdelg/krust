use crate::gpio::GPIO;
use crate::timer::SysTimer;
use crate::asm::{
    PUT32,
    GET32,
    dev_barrier
};

const CLOCK_SPEED_MHZ: u32 = 700;
const TX: u32 = 14;
const RX: u32 = 15;
const BAUD: u32 = 115200;
// const CYCLES_PER_BIT: u32 = CLOCK_SPEED_MHZ * 1000 * 1000 / BAUD;

pub struct SWUART {
    
}

impl SWUART {
    pub fn get8(&self) -> u8 {
        1
    }

    pub fn put_str(gpio: &GPIO, msg: &'static str) {
        for b in msg.bytes() {
            SWUART::put8(gpio, b);
        }
    }

    pub fn put8(gpio: &GPIO, byte: u8) {
        
        let start = SysTimer::get_cycles();
        // let cycles_per_bit = CLOCK_SPEED_MHZ * 1000 * 1000 / BAUD;
        let cycles_per_bit = 6076;
        // let cycles_per_bit = 700 * 1000 * 1000;
        let mut delay = cycles_per_bit;
        
        // Start bit
        gpio.clear(TX);
        SysTimer::delay_cycles_exact(start, delay);
        delay += cycles_per_bit;

        // return;

        // Byte
        gpio.set_val(TX, ((byte >> 0) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 1) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 2) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 3) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 4) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 5) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 6) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;
        gpio.set_val(TX, ((byte >> 7) & 1) as u32); SysTimer::delay_cycles_exact(start, delay); delay += cycles_per_bit;

        // Stop bit
        gpio.set(TX);
        SysTimer::delay_cycles_exact(start, delay);
    }

    pub fn init(gpio: &GPIO) {
        // Annoying stuff to disable uart
        // dev_barrier();

        // let mut aux_disable = unsafe { GET32(0x20215000 + 0x4) };
        // aux_disable &= !0b1;

        // unsafe { PUT32(0x20215000 + 0x4, aux_disable); }

        // dev_barrier();

        // unsafe { PUT32(0x2020_0094, 0); }

        gpio.set_output(TX);
        gpio.set_input(RX);

        gpio.set(TX);
    }
}