use crate::pi::{calculate_pi, Target};

mod pi;

fn main() {
    let mut target = Target::default();
    let mut total_iterations = 0i64;
    loop {
        let pi = calculate_pi(&mut target, 100_000);
        total_iterations += 100_000;
        println!("[iter: {total_iterations}] {pi}")
    }
}
