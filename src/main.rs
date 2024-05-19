#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
use panic_rtt_target as _;
extern crate stm32l4xx_hal as hal;
use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");

    rprintln!("Configuring peripherals...");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    rprintln!("Configuring clock...");
    let clocks = rcc.cfgr.sysclk(64.MHz()).freeze(&mut flash.acr, &mut pwr);

    rprintln!("Configuring GPIO...");
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb5
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    rprintln!("Configuring timer...");
    let mut timer = Delay::new(cp.SYST, clocks);

    rprintln!("Configured!");
    rprintln!("Starting Loop.");
    loop {
        timer.delay_ms(1000_u32);
        led.set_high();
        rprintln!("On");
        timer.delay_ms(1000_u32);
        led.set_low();
        rprintln!("Off");
    }
}
