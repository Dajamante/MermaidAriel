#![no_main]
#![no_std]
use ariel_os::debug::log::info;

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    hal::peripherals,
    time::Timer,
};
ariel_os::hal::define_peripherals!(Peripherals {
    led: GPIO7,
    btn: GPIO9,
});

// TODO:
// 1. laze-project file, config this
// 2. find out what is going on here
//  cargo add embassy-futures
//     Updating crates.io index
//       Adding embassy-futures v0.1.1 to dependencies
//              Features:
//              - defmt
//              - log
//     Updating crates.io index
// error: failed to select a version for `esp-hal`.
//     ... required by package `ariel-os-esp v0.2.0 (/Users/ai/Embedded/mermaidariel/build/imports/ariel-os/src/ariel-os-esp)`
//     ... which satisfies path dependency `ariel-os-esp` (locked to 0.2.0) of package `ariel-os-hal v0.2.0 (/Users/ai/Embedded/mermaidariel/build/imports/ariel-os/src/ariel-os-hal)`
//     ... which satisfies path dependency `ariel-os-hal` (locked to 0.2.0) of package `ariel-os-embassy v0.2.0 (/Users/ai/Embedded/mermaidariel/build/imports/ariel-os/src/ariel-os-embassy)`
//     ... which satisfies path dependency `ariel-os-embassy` (locked to 0.2.0) of package `ariel-os v0.2.0 (/Users/ai/Embedded/mermaidariel/build/imports/ariel-os/src/ariel-os)`
//     ... which satisfies path dependency `ariel-os` (locked to 0.2.0) of package `mermaidariel v0.1.0 (/Users/ai/Embedded/mermaidariel)`
// versions that meet the requirements `^0.23.1` are: 0.23.1

// package `ariel-os-esp` depends on `esp-hal` with feature `optfield` but `esp-hal` does not have that feature.
#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: Peripherals) {
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
            embassy_futures::select::select(btn.wait_for_low(), Timer::after_millis(13300)).await;

        led.toggle();
    }
    // loop {
    //     if btn.is_high() {
    //         info!("Button pressed");
    //         led.toggle();
    //     }
}
