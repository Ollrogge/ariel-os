#![no_std]

use ariel_os_debug::log::debug;

pub fn init() {
    debug!("thingy53::init()");
    nrf5340::init();
}
