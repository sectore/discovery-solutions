#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let tick_period = 50_u16; // 50ms
    let max_leds = leds.len();
    let max_ticks = max_leds * 2;
    let mut tick_counter = 0;

// 1 tick == 50ms

// ticks    |0|1|2|3|4|5|6|7|8|9|0|1|2|3|4|5|
// led 0    |x|x|x|-|-|-|-|-|-|-|-|-|-|-|-|-|
// led 1    |-|-|x|x|x|-|-|-|-|-|-|-|-|-|-|-|
// led 2    |-|-|-|-|x|x|x|-|-|-|-|-|-|-|-|-|
// led 3    |-|-|-|-|-|-|x|x|x|-|-|-|-|-|-|-|
// led 4    |-|-|-|-|-|-|-|-|x|x|x|-|-|-|-|-|
// led 5    |-|-|-|-|-|-|-|-|-|-|x|x|x|-|-|-|
// led 6    |-|-|-|-|-|-|-|-|-|-|-|-|x|x|x|-|
// led 7    |x|-|-|-|-|-|-|-|-|-|-|-|-|-|x|x|

    loop {
        if tick_counter % 2 == 0 // even tick numbers
        { 
            let on_index = tick_counter / 2;
            leds[on_index].on();

        } else // odd tick numbers
        {
            let off_index = if tick_counter >= 3 { 
                    (tick_counter - 3) / 2 
                } 
                else {
                    max_leds - 1
                };
            leds[off_index].off();

        }
        delay.delay_ms(tick_period); 
        tick_counter = (tick_counter + 1) % max_ticks;

    }
}
