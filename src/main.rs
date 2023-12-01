#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Pull, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _}; // global logger

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut serial = Flex::new(p.PA9);
    serial.set_as_input_output(Speed::VeryHigh, Pull::None);

    loop {
        info!("tick");
        Timer::after_millis(300).await;
    }
}
