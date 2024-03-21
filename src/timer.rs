use crate::asm::{
    GET32,
    PUT32,
    get_cycles_asm,
    cycle_init_asm
};
  

const SYS_TIMER_BASE: u32 = 0x2000_3004;

pub struct SysTimer {
}

impl SysTimer {
    pub fn delay_usec(usecs: u32) {
        let t_init = unsafe { GET32(SYS_TIMER_BASE) };
        let mut t_now = t_init;
        
        while t_now - t_init < usecs {
            t_now = unsafe { GET32(SYS_TIMER_BASE) };
        }
    }

    pub fn cycle_init() {
        unsafe { cycle_init_asm(); }
    }

    pub fn get_cycles() -> u32 {
        unsafe { get_cycles_asm() }
    }

    pub fn delay_cycles_exact(start: u32, delay: u32) {
        loop {
            let cycles = SysTimer::get_cycles();

            if cycles - start >= delay {
                break;
            }
        }
    }
}