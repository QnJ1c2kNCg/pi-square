use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use num_format::{Locale, ToFormattedString};

use clap::{arg, command, Parser};
use uptime::Uptime;

use crate::display::Display;
use crate::pi::{calculate_pi, Target};

mod display;
mod pi;
mod uptime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Screen refresh rate (in seconds)
    #[arg(short, long, default_value_t = 5)]
    refresh_interval_secs: u64,

    /// Iteration increment to calculate pi
    #[arg(short, long, default_value_t = 100_000)]
    pi_iteration: u64,
}

fn main() {
    let args = Args::parse();

    let pi = Arc::new(RwLock::new(0.0f64));
    let pi_clone = Arc::clone(&pi);

    let total_iter = Arc::new(AtomicU64::new(0));
    let total_iter_clone = Arc::clone(&total_iter);

    let uptime = Uptime::new();

    let mut display = Display::new();

    std::thread::spawn(move || {
        let mut target = Target::default();
        loop {
            *pi_clone.write().unwrap() = calculate_pi(&mut target, args.pi_iteration);
            total_iter_clone.fetch_add(args.pi_iteration, Ordering::Relaxed);
        }
    });

    loop {
        std::thread::sleep(Duration::from_secs(args.refresh_interval_secs));
        let pi = &format!("Pi: {:.13}", pi.read().unwrap());
        let iter = &format!(
            "Iter: {}",
            total_iter
                .load(Ordering::Relaxed)
                .to_formatted_string(&Locale::en)
        );
        let uptime = &format!(
            "Uptime: {} hrs",
            (uptime.get().as_secs() / 3600).to_formatted_string(&Locale::en)
        );
        display.print(pi, "", iter, uptime);

        println!("[iter: {iter}] {pi}, program uptime: {uptime}");
    }
}
