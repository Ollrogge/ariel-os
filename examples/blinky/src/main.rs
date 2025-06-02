#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    gpio::{Level, Output},
    time::Timer,
};

use ariel_os::debug::{ExitCode, exit, log::*};
#[cfg(context = "nrf5340")]
use embassy_nrf::{pac, reset};

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::LedPeripherals) {
    let mut led = Output::new(peripherals.led, Level::Low);

    cfg_if::cfg_if! {
        if #[cfg(context="nrf5340")] {
            let p0 = pac::P0;
            p0.pin_cnf(29).write(|w| w.set_mcusel(pac::gpio::vals::Mcusel::NETWORK_MCU));

            reset::release_network_core();
        }
    }

    // The micro:bit uses an LED matrix; pull the column line low.
    #[cfg(context = "bbc-microbit-v2")]
    let _led_col1 = Output::new(peripherals.led_col1, Level::Low);

    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
