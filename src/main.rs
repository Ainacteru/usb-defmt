#![no_std]
#![no_main]

use atsamd_hal::{
    clock::GenericClockController, delay::Delay, ehal::delay::DelayNs, gpio::{Output, PA17, Pin}, pac::{CorePeripherals, Interrupt, NVIC, Peripherals}, prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin
};
use cortex_m::peripheral::scb::Exception::SysTick;
use cortex_m_rt::entry;
use defmt::{info};

use embassy_executor::Spawner;
use embassy_time::Timer;
use samd21_usb_defmt::{Pins, time_driver, usb::Usb};
use samd21_usb_defmt::timer;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = Pins::new(peripherals.port);


    Usb::set_up(&mut clocks, &mut peripherals.pm, pins.usb_dm, pins.usb_dp, peripherals.usb);
    // timer::set_up(&mut clocks, peripherals.tc3, &mut peripherals.pm);
    time_driver::init(peripherals.tc3, &mut peripherals.pm, &mut clocks);

    enable_interrupts();

    let mut led = pins.led.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    spawner.spawn(blink(led).unwrap());

    loop {
        info!("hello");
        Timer::after_millis(1000).await;
    }
}

fn enable_interrupts() {
    unsafe {
        NVIC::unmask(Interrupt::USB);
        NVIC::unmask(Interrupt::TC3);
        NVIC::unmask(Interrupt::SERCOM3);
    }
}

#[embassy_executor::task]
async fn blink(mut pin: Pin<PA17, Output<atsamd_hal::gpio::PushPull>>) {
    loop {
        pin.toggle();
        Timer::after_millis(500).await;
    }
}
