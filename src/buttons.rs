use embassy_stm32::gpio::Input;
use embassy_time::{Duration, Timer};

use crate::debouncer::DebouncedButton;

const NUM_BUTTONS: usize = 4;

#[embassy_executor::task]
pub async fn buttons_task(buttons: [Input<'static>; NUM_BUTTONS]) {
    let mut debounced_buttons = buttons.map(DebouncedButton::new);

    loop {
        for button in &mut debounced_buttons {
            button.poll();
        }

        Timer::after(Duration::from_hz(50)).await;
    }
}
