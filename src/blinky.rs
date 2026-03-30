use embassy_stm32::gpio::Output;
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn blinky_task(mut pin: Output<'static>) {
    loop {
        pin.toggle();

        Timer::after_secs(1).await;
    }
}
