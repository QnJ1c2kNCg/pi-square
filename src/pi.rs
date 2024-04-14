use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
struct Coords {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum OnTarget {
    Yes,
    No,
}

#[derive(Default)]
pub struct Target {
    hit_count: f64,
    miss_count: f64,
}

impl Target {
    fn on_target(coords: Coords) -> OnTarget {
        let distance_from_center = f64::sqrt(coords.x.powi(2) + coords.y.powi(2));
        if distance_from_center <= 1.0 {
            OnTarget::Yes
        } else {
            OnTarget::No
        }
    }

    fn update(&mut self, _dart: Dart) {
        // random number between -1 and 1
        let between = Uniform::from(-1.0..=1.0);
        let mut rng = rand::thread_rng();
        let x: f64 = between.sample(&mut rng);
        let y: f64 = between.sample(&mut rng);
        let landing_coords = Coords { x, y };

        match Target::on_target(landing_coords) {
            OnTarget::Yes => self.hit_count += 1.0,
            OnTarget::No => self.miss_count += 1.0,
        }
    }

    fn what_is_life(&self) -> f64 {
        const SQUARE_AREA: f64 = 4.0;
        self.hit_count * SQUARE_AREA / (self.hit_count + self.miss_count)
    }
}

struct Dart;
impl Dart {
    fn throw(self, target: &mut Target) {
        target.update(self);
    }
}

pub fn calculate_pi(target: &mut Target, iteration: u64) -> f64 {
    for _ in 0..iteration {
        let dart = Dart;
        dart.throw(target);
    }
    target.what_is_life()
}
