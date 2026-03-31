#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config,
    adc::AdcChannel,
    bind_interrupts, dma,
    gpio::{Input, Level, Output, Pull, Speed},
    i2c::{self, I2c},
    peripherals::{self, DMA1_CH2, DMA1_CH3},
};
use embassy_time::Timer;
use panic_probe as _;

use crate::{
    blinky::blinky_task, buttons::buttons_task, display::display_task, joystick::joystick_task,
    rgb::rgb_task,
};

pub mod blinky;
pub mod buttons;
pub mod debouncer;
pub mod display;
pub mod joystick;
pub mod rgb;

bind_interrupts!(struct Irqs {
    I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<DMA1_CH2>, dma::InterruptHandler<DMA1_CH3>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    // Blinky task setup
    let led = Output::new(p.PA5, Level::Low, Speed::Low);
    spawner.spawn(blinky_task(led)).unwrap();

    // Buttons task setup
    let button_up = Input::new(p.PC11, Pull::Down);
    let button_down = Input::new(p.PC1, Pull::Down);
    let button_left = Input::new(p.PC13, Pull::Up);
    let button_right = Input::new(p.PC10, Pull::Down);

    spawner
        .spawn(buttons_task([
            button_up,
            button_down,
            button_left,
            button_right,
        ]))
        .unwrap();

    // Joystick task setup
    spawner
        .spawn(joystick_task(
            p.ADC1,
            p.DMA1_CH1,
            p.PC5.degrade_adc(),
            p.PC4.degrade_adc(),
        ))
        .unwrap();

    // Display task setup
    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        p.DMA1_CH2,
        p.DMA1_CH3,
        Irqs,
        Default::default(),
    );

    spawner.spawn(display_task(i2c)).unwrap();

    let up_led = Output::new(p.PC6, Level::High, Speed::Low);
    let down_led = Output::new(p.PC2, Level::High, Speed::Low);
    let left_led = Output::new(p.PF3, Level::High, Speed::Low);
    let right_led = Output::new(p.PC12, Level::High, Speed::Low);

    let red = Output::new(p.PD3, Level::Low, Speed::VeryHigh);
    let green = Output::new(p.PD2, Level::Low, Speed::VeryHigh);
    let blue = Output::new(p.PD4, Level::Low, Speed::VeryHigh);

    spawner
        .spawn(rgb_task(
            [red, green, blue],
            [up_led, down_led, left_led, right_led],
        ))
        .unwrap();

    loop {
        Timer::after_millis(50).await;
    }
}
