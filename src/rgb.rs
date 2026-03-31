use core::cell::Cell;

use critical_section::Mutex;
use embassy_stm32::gpio::{Level, Output};
use embassy_time::{Duration, Timer};

static LED_STATE: Mutex<Cell<[Colour; 4]>> = Mutex::new(Cell::new([Colour::black(); 4]));

#[derive(Copy, Clone)]
pub struct Colour {
    red: bool,
    green: bool,
    blue: bool,
}

impl Colour {
    fn to_array(self) -> [bool; 3] {
        [self.red, self.green, self.blue]
    }
}

impl Colour {
    pub const fn white() -> Self {
        Self {
            red: true,
            green: true,
            blue: true,
        }
    }

    pub const fn black() -> Self {
        Self {
            red: false,
            green: false,
            blue: false,
        }
    }

    pub const fn red() -> Self {
        Self {
            red: true,
            green: false,
            blue: false,
        }
    }

    pub const fn green() -> Self {
        Self {
            red: false,
            green: true,
            blue: false,
        }
    }

    pub const fn blue() -> Self {
        Self {
            red: false,
            green: false,
            blue: true,
        }
    }
}

pub enum Led {
    Up,
    Down,
    Left,
    Right,
}

pub fn set_led(led: Led, colour: Colour) {
    critical_section::with(|cs| {
        let mut leds = LED_STATE.borrow(cs).get();

        leds[led as usize] = colour;

        LED_STATE.borrow(cs).set(leds);
    });
}

#[embassy_executor::task]
pub async fn rgb_task(mut rgb_pins: [Output<'static>; 3], mut led_pins: [Output<'static>; 4]) {
    for pin in &mut led_pins {
        pin.set_high();
    }

    loop {
        let state = critical_section::with(|cs| LED_STATE.borrow(cs).get());

        for i in 0..4 {
            led_pins[i].set_level(Level::Low);

            let current_state = state[i].to_array();

            for i in 0..3 {
                rgb_pins[i].set_level(Level::from(current_state[i]))
            }

            Timer::after(Duration::from_hz(200)).await;

            led_pins[i].set_level(Level::High);
        }
    }
}
