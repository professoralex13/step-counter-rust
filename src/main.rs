#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config,
    adc::AdcChannel,
    gpio::{Input, Level, Output, Pull, Speed},
    i2c::{self, I2c},
};
use panic_probe as _;

use crate::{
    blinky::blinky_task, buttons::buttons_task, display::display_task, joystick::joystick_task,
};

pub mod blinky;
pub mod buttons;
pub mod debouncer;
pub mod display;
pub mod joystick;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let led = Output::new(p.PA5, Level::Low, Speed::Low);

    let button_down = Input::new(p.PC1, Pull::Down);
    let button_up = Input::new(p.PC11, Pull::Down);
    let button_right = Input::new(p.PC10, Pull::Down);
    let button_left = Input::new(p.PC13, Pull::Up);

    spawner.spawn(blinky_task(led)).unwrap();
    spawner
        .spawn(buttons_task([
            button_down,
            button_up,
            button_left,
            button_right,
        ]))
        .unwrap();

    spawner
        .spawn(joystick_task(
            p.ADC1,
            p.DMA1_CH1,
            p.PC5.degrade_adc(),
            p.PC4.degrade_adc(),
        ))
        .unwrap();

    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, i2c::Config::default());

    spawner.spawn(display_task(i2c)).unwrap();

    loop {}
}
