use uptime::Uptime;

use crate::pi::{calculate_pi, Target};

mod pi;
mod uptime;

fn main() {
    let uptime = Uptime::new();
    let mut target = Target::default();
    let mut total_iterations = 0i64;
    loop {
        let pi = calculate_pi(&mut target, 1_000_000);
        total_iterations += 1_000_000;
        println!(
            "[iter: {total_iterations}] {pi}, program uptime: {:?}",
            uptime.get()
        )
    }
}
