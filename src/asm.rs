extern "C" {
    pub fn GET32(addr: u32) -> u32;

    pub fn PUT32(addr: u32, val: u32);

    pub fn cycle_init_asm();
    pub fn get_cycles_asm() -> u32;

    fn dev_barrier_asm();
}

pub fn dev_barrier() {
    unsafe { dev_barrier_asm() }
}