#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    Config,
    gpio::{Input, Level, Output, Pull, Speed},
};

use crate::{blinky::blinky_task, buttons::buttons_task};

use {defmt_rtt as _, panic_probe as _};

pub mod blinky;
pub mod buttons;
pub mod debouncer;

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

    loop {}
}
