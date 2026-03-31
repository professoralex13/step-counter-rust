use embassy_stm32::gpio::Input;
use embassy_time::{Duration, Timer};

use crate::{
    debouncer::DebouncedButton,
    rgb::{Colour, Led, set_led},
};

const NUM_BUTTONS: usize = 4;

#[embassy_executor::task]
pub async fn buttons_task(buttons: [Input<'static>; NUM_BUTTONS]) {
    let [up, down, left, right] = buttons;

    let mut debounced_buttons = [
        DebouncedButton::new(up, true),
        DebouncedButton::new(down, true),
        DebouncedButton::new(left, false),
        DebouncedButton::new(right, true),
    ];

    let mut left_led_on = false;
    let mut right_led_on = false;

    loop {
        for button in &mut debounced_buttons {
            button.poll();
        }

        if debounced_buttons[2].just_changed_to(true) {
            left_led_on = !left_led_on;

            set_led(
                Led::Left,
                if left_led_on {
                    Colour::white()
                } else {
                    Colour::black()
                },
            );
        }

        if debounced_buttons[3].just_changed_to(true) {
            right_led_on = !right_led_on;

            set_led(
                Led::Right,
                if right_led_on {
                    Colour::white()
                } else {
                    Colour::black()
                },
            );
        }

        Timer::after(Duration::from_hz(50)).await;
    }
}
