// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
};
use nb::block;

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);
    
    loop{
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
        block!(timer.wait()).unwrap();
    }
}
