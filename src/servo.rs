use crate::gpio::GPIO;
use crate::timer::SysTimer;

pub fn pwm(gpio_instance: &GPIO) {
    let hz = 50;
    let on_us = 2000;
    let cycle_len_us = (1000 * 1000) / 50;

    let hi_period_us = on_us;
    let low_period_us = cycle_len_us - on_us;

    gpio_instance.set_output(20);
    gpio_instance.clear(20);

    for _ in 0..1000 {
        gpio_instance.set(20);
        SysTimer::delay_usec(on_us);

        gpio_instance.clear(20);
        SysTimer::delay_usec(low_period_us);
    }
}