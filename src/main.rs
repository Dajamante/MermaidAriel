#![no_main]
#![no_std]
use ariel_os::debug::log::{debug, info};

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    hal::peripherals,
    time::Timer,
};
#[cfg(context = "espressif-esp32-c3-lcdkit")]
ariel_os::hal::define_peripherals!(Peripherals {
    led: GPIO7,
    btn: GPIO9,
});

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led: P0_13,
    btn: P0_11,
});

// TODO:
// 1. laze-project file, config this
#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: Peripherals) {
    // debug!("Inside main");
    let mut led = Output::new(peripherals.led, Level::Low);
    // let mut btn = Input::new(peripherals.btn, Pull::Up);
    let mut btn = Input::builder(peripherals.btn, Pull::Up)
        .build_with_interrupt()
        .unwrap();
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );
    loop {
        let _ =
        // remove that silly select
            // embassy_futures::poll_once(btn.wait_for_low()).await;
        embassy_futures::select::select(btn.wait_for_low(), Timer::after_millis(13300)).await;

        led.toggle();
    }
    // loop {
    //     if btn.is_high() {
    //         info!("Button pressed");
    //         led.toggle();
    //     }
}
