#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;
use rp_pico::hal::clocks::Clock;
use rp_pico::hal::gpio::{Pin, PinId, PushPullOutput};

#[entry]
fn main() -> ! {
    defmt::info!("main start");

    let mut pac = rp_pico::hal::pac::Peripherals::take().unwrap();
    let core = rp_pico::hal::pac::CorePeripherals::take().unwrap();
    let mut watchdog = rp_pico::hal::watchdog::Watchdog::new(pac.WATCHDOG);

    let clocks = rp_pico::hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let sio = rp_pico::hal::sio::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led_board = pins.led.into_push_pull_output();
    // let led_ext = pins.gpio28.into_push_pull_output();

    runloop(led_board, delay)
}

#[inline(never)]
fn runloop<I: PinId>(mut led: Pin<I, PushPullOutput>, mut delay: cortex_m::delay::Delay) -> ! {
    defmt::info!("runloop start");

    let morse_unit = 120;

    loop {
        defmt::debug!("morse: S");
        for _ in 1..=3 {
            led.set_high().unwrap();
            delay.delay_ms(1 * morse_unit);
            led.set_low().unwrap();
            delay.delay_ms(1 * morse_unit);
        }
        delay.delay_ms(2 * morse_unit);
        defmt::debug!("morse: O");
        for _ in 1..=3 {
            led.set_high().unwrap();
            delay.delay_ms(3 * morse_unit);
            led.set_low().unwrap();
            delay.delay_ms(1 * morse_unit);
        }
        delay.delay_ms(2 * morse_unit);
        defmt::debug!("morse: S");
        for _ in 1..=3 {
            led.set_high().unwrap();
            delay.delay_ms(1 * morse_unit);
            led.set_low().unwrap();
            delay.delay_ms(1 * morse_unit);
        }
        delay.delay_ms(6 * morse_unit);
    }
}
