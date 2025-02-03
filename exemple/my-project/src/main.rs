//! This app prints "Hello, Rust!" to the console then exits.

#![no_main]
#![no_std]

extern crate flipperzero_rt;

use core::ffi::CStr;

use flipperzero::println;
use flipperzero_rt::{entry, manifest};

manifest!(
    name = "Flipper Zero Rust",
    app_version = 1,
    has_icon = true,
    icon = "rustacean-10x10.icon",
);

entry!(main);

// screen /dev/ttyACM0 115200
// Pour voir les logs de ton app
// storage send target/thumbv7em-none-eabihf/debug/my-project.fap /ext/apps/Examples/my-project.fap
// Pour envoyer le .fap directement sur le flipper!
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!");

    return 0;
}
