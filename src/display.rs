use embassy_stm32::{
    i2c::{I2c, Master},
    mode::Blocking,
};
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::format;
use ssd1306::{
    I2CDisplayInterface, Ssd1306, mode::DisplayConfig, prelude::DisplayRotation,
    size::DisplaySize128x64,
};

use crate::joystick::get_joystick_receiver;

const CENTER_DEADZONE: f32 = 0.05;

enum ValueState {
    Rest,
    Low(f32),
    High(f32),
}

impl ValueState {
    fn from_ratio(value: f32) -> Self {
        let absolute = value.abs();

        if absolute < CENTER_DEADZONE {
            Self::Rest
        } else if value.is_sign_positive() {
            Self::High(absolute)
        } else {
            Self::Low(absolute)
        }
    }
}

#[embassy_executor::task]
pub async fn display_task(i2c: I2c<'static, Blocking, Master>) {
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut joystick_receiver = get_joystick_receiver().unwrap();

    loop {
        display.clear(BinaryColor::Off).unwrap();

        let joystick_state = joystick_receiver.get().await;

        let [x_state, y_state] = joystick_state.map(ValueState::from_ratio);

        let x_display = match x_state {
            ValueState::Rest => format!(20; "Rest"),
            ValueState::Low(value) => format!(20; "Up: {}%", (value * 100.0) as i32),
            ValueState::High(value) => format!(20; "Down: {}%", (value * 100.0) as i32),
        }
        .unwrap();

        let y_display = match y_state {
            ValueState::Rest => format!(20; "Rest"),
            ValueState::Low(value) => format!(20; "Right: {}%", (value * 100.0) as i32),
            ValueState::High(value) => format!(20; "Left: {}%", (value * 100.0) as i32),
        }
        .unwrap();

        Text::with_baseline(
            x_display.as_str(),
            Point::new(0, 10),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        Text::with_baseline(
            y_display.as_str(),
            Point::new(0, 20),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();

        Timer::after(Duration::from_hz(10)).await;
    }
}
