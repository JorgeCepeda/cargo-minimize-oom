#[macro_use]
extern crate tracing;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::{thread, time};

use anyhow::Result;
use cargo_minimize_oom::{Cargo, Parser};
use tracing::{Level, error};

fn main() -> Result<()> {
    let Cargo::Minimize_OOM(options) = Cargo::parse();

    cargo_minimize_oom::init_recommended_tracing_subscriber(Level::INFO);

    let cancel = Arc::new(AtomicBool::new(false));
    let cancel2 = Arc::clone(&cancel);

    let mut ctrl_c_pressed = false;
    let result = ctrlc::set_handler(move || {
        // If ctrl c was pressed already, kill it now.
        if ctrl_c_pressed {
            warn!("Process killed");
            std::process::exit(130);
        }

        warn!("Shutting down gracefully, press CTRL-C again to kill");
        cancel.store(true, Ordering::SeqCst);
        ctrl_c_pressed = true;
        cargo_minimize_oom::stop_keyboard_io();
    });

    if let Err(err) = result {
        error!("Failed to install CTRL-C handler: {err}");
    }

    let amount = time::Duration::from_secs(4);
    thread::sleep(amount);
    let res = cargo_minimize_oom::minimize(options, cancel2);
    cargo_minimize_oom::stop_keyboard_io();
    res
}
