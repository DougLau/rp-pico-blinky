#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_halt as _;
use rp2040_hal::{clocks::Clock, pac, sio::Sio, watchdog::Watchdog};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    const EXTERNAL_CRYSTAL_FREQ_HZ: u32 = 12_000_000;
    let clocks = rp2040_hal::clocks::init_clocks_and_plls(
        EXTERNAL_CRYSTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(
        core.SYST,
        clocks.system_clock.freq().integer(),
    );

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(250);
    }
}
