#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, error};
use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Flex, Pull, Speed}, i2c::{I2c, self, Error, Config}, bind_interrupts, peripherals, dma::NoDma, time::Hertz};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _}; // global logger

use mcp7940n::{Mcp7940n, ClockConfig, ClockSource};
use chrono::Timelike as _;


const ADDRESS: u8 = 0b110_1111;
const READ_MASK: u8 = 0b0000_0001;


bind_interrupts!(struct Irqs {
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // let mut serial = Flex::new(p.PA9);
    // serial.set_as_input_output(Speed::VeryHigh, Pull::None);

    let i2c = I2c::new(
        p.I2C2,
        p.PB10,
        p.PB3,
        Irqs,
        NoDma,
        NoDma,
        Hertz(10_000),
        Default::default()
    );

    let mut clock = mcp7940n::Mcp7940n::new(i2c);

    clock.configure_clock(&ClockConfig {
        enabled: true, 
        clock_source: ClockSource::ExtCrystal 
    }).unwrap();

    loop {
        info!("waiting for clock to start...");
        if Ok(true) == clock.osc_running() {
            break;
        }
        Timer::after_millis(1000).await;
    }

    info!("yeah!!!");

    loop {
        let now = clock.now().unwrap();
        info!("now: {}:{}:{}", now.time().hour(), now.time().minute(), now.time().second());
        Timer::after_millis(1000).await;
    }

    // let mut data = [0u8; 1];

    // let mut current_data = [0u8; 9];

    // if let Err(e) = i2c.blocking_write_read(ADDRESS, &[0x00], &mut current_data[1..]) {
    //     error!("error0: {}", e);
    // }

    // // set flags, I guess? 
    // current_data[1] |= 0b1000_0000; // ST bit (clock enable)
    // // current_data[3] |= 
    // current_data[8] &= 0b1111_0111; // External clock enable

    // if let Err(e) = i2c.blocking_write(ADDRESS,&current_data) {
    //     error!("error0: {}", e);
    // }

    // // match i2c.blocking_write_read(ADDRESS, &[0x00], &mut data) {
    // //     Ok(()) => info!("{}: {:#010b}", 0x00, data[0]),
    // //     Err(Error::Timeout) => error!("Operation timed out"),
    // //     Err(e) => error!("I2c Error: {:?}", e),  
    // // }

    // loop {
    //     info!("waiting for clock to be enabled...");
    //     let mut clock_info = [0u8; 1];
    //     match i2c.blocking_write_read(ADDRESS, &[0x03], &mut clock_info) {
    //         Ok(()) => {
    //             if clock_info[0] & 0b0010_0000 != 0 {
    //                 break;
    //             } else {
    //                 info!("clock not started...");
    //             }
    //         },
    //         Err(Error::Timeout) => error!("Operation timed out"),
    //         Err(e) => error!("I2c Error: {:?}", e),  
    //     }
    //     Timer::after_millis(1000).await;
    // }

    // loop {
    //     info!("trying to read...");
    //     let mut time_data = [0u8; 8];
    //     match i2c.blocking_write_read(ADDRESS, &[0x00], &mut time_data) {
    //         Ok(()) => info!("time: {:#010b}", time_data),
    //         Err(Error::Timeout) => error!("Operation timed out"),
    //         Err(e) => error!("I2c Error: {:?}", e),  
    //     }
    //     Timer::after_millis(1000).await;
    // }
}
