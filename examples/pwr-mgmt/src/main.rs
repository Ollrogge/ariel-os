#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

use ariel_os::{
    debug::{exit, log::*, ExitCode},
    pm,
    time::{Duration, Timer},
};

#[ariel_os::task(autostart)]
async fn main() {
    info!("Rebooting in 3 sec");

    Timer::after(Duration::from_secs(3)).await;
    pm::reboot();
}
