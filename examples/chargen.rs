// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
    serial::Serial,
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

    // Prepare the alternate function I/O registers
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
//    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    let serial = Serial::usart1(
        dp.USART1, (tx, rx), &mut afio.mapr, 115200.bps(), clocks, &mut rcc.apb2,
    );

    let (mut tx, mut _rx) = serial.split();

    loop{
        led.set_high();
        block!(tx.write(b'A')).ok();
        block!(timer.wait()).unwrap();
        led.set_low();
        block!(tx.write(b'B')).ok();
        block!(timer.wait()).unwrap();
    }
}
