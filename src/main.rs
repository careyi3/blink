#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::rtc::Rtc;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");

    if let (Some(mut dp), Some(_)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        rprintln!("Configuring clock...");
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        Rtc::new(dp.RTC, &mut dp.PWR);

        rprintln!("Configuring timer...");
        let mut delay = dp.TIM5.delay_us(&clocks);

        loop {
            rprintln!("Toggle");
            led.toggle();
            delay.delay_ms(1000);
        }
    }

    loop {}
}
