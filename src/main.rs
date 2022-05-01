#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use rp_pico::hal::clocks::Clock;

#[entry]
fn main() -> ! {
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let sio = rp_pico::hal::sio::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_board = pins.led.into_push_pull_output();
    let mut led_ext = pins.gpio28.into_push_pull_output();

    loop {
        led_board.set_high().unwrap();
        led_ext.set_low().unwrap();
        delay.delay_ms(500);
        led_board.set_low().unwrap();
        led_ext.set_high().unwrap();
        delay.delay_ms(500);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
