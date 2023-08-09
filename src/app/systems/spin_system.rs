use std::f64::consts::PI;
use specs::{Join, Read, ReadStorage, System};
use crate::app::components::now_in_seconds::NowInSeconds;
use crate::app::components::spin_target::SpinTarget;

pub struct SpinSystem;

const RPM: f64 = 2. * PI;
const SPIN_RPM: f64 = 6. * RPM;

impl<'a> System<'a> for SpinSystem {
	type SystemData = (Read<'a, NowInSeconds>, ReadStorage<'a, SpinTarget>);
	fn run(&mut self, (now_in_seconds, spin_target): Self::SystemData) {
		let now_in_minutes = now_in_seconds.0 / 60.;
		let travel = SPIN_RPM * now_in_minutes;
		let (x, y) = (travel, 3. * travel);
		for spin_target in spin_target.join() {
			spin_target.euler.set(x, y, 0.);
		}
	}
}
