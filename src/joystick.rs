use embassy_stm32::{
    Peri,
    adc::{Adc, AnyAdcChannel, Resolution, SampleTime},
    bind_interrupts, dma,
    peripherals::{ADC1, DMA1_CH1},
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{Receiver, Watch},
};
use embassy_time::{Duration, Timer};

static JOYSTICK_VALUES: Watch<CriticalSectionRawMutex, [f32; 2], 2> = Watch::new();

pub fn get_joystick_receiver() -> Option<Receiver<'static, CriticalSectionRawMutex, [f32; 2], 2>> {
    JOYSTICK_VALUES.receiver()
}

const JOYSTICK_HIGH: u16 = 4030;
const JOYSTICK_LOW: u16 = 370;

fn transform_raw(raw: u16) -> f32 {
    if raw < JOYSTICK_LOW {
        -1.0
    } else if raw > JOYSTICK_HIGH {
        1.0
    } else {
        2.0 * (f32::from(raw - JOYSTICK_LOW) / f32::from(JOYSTICK_HIGH - JOYSTICK_LOW)) - 1.0
    }
}

bind_interrupts!(struct Irqs {
    DMA1_CHANNEL1 => dma::InterruptHandler<DMA1_CH1>;
});

#[embassy_executor::task]
pub async fn joystick_task(
    adc: Peri<'static, ADC1>,
    mut dma: Peri<'static, DMA1_CH1>,
    mut x_pin: AnyAdcChannel<'static, ADC1>,
    mut y_pin: AnyAdcChannel<'static, ADC1>,
) {
    let mut adc = Adc::new(adc, Resolution::BITS12);

    let sender = JOYSTICK_VALUES.sender();

    loop {
        let mut measurements = [0u16; 2];

        adc.read(
            dma.reborrow(),
            Irqs,
            [
                (&mut x_pin, SampleTime::CYCLES2_5),
                (&mut y_pin, SampleTime::CYCLES2_5),
            ]
            .into_iter(),
            &mut measurements,
        )
        .await;

        sender.send(measurements.map(transform_raw));

        Timer::after(Duration::from_hz(50)).await;
    }
}
