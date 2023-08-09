pub mod three;
pub mod dom;
pub mod basics;
pub mod four;

use std::f64::consts::PI;
use specs::{Builder, Component, DispatcherBuilder, Join, Read, ReadStorage, System, World, WorldExt};
use specs::storage::BTreeStorage;
use dom::Window;
use crate::dom::{Error};
use crate::four::RenderSystem;
use crate::three::{Euler};

#[derive(Component)]
#[storage(BTreeStorage)]
struct SpinTarget {
	euler: Euler,
}

#[derive(Default)]
struct NowInSeconds(f64);

struct SpinSystem;

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

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");

	let mesh = {
		let geometry = three::BoxGeometry::new(2., 1., 1.);
		let material = three::MeshBasicMaterial::new();
		material.color().set(0., 0., 255.);
		three::Mesh::new(geometry, material)
	};
	mesh.position().set_xyz(0., 1.6, -3.);
	mesh.rotation().set(0., 0., 0.);

	let renderer = four::Renderer::new(&Window::connect());
	renderer.scene().add(&mesh);

	let mut world = World::new();
	world.register::<SpinTarget>();
	world.insert(NowInSeconds(0.));
	world.insert(renderer.clone());
	world.create_entity().with(SpinTarget { euler: mesh.rotation() }).build();

	let mut dispatcher = DispatcherBuilder::new()
		.with_thread_local(SpinSystem)
		.with_thread_local(RenderSystem)
		.build();

	renderer.set_animation_loop_with_forget(move |time| {
		*world.write_resource::<NowInSeconds>() = NowInSeconds(time / 1000.);
		dispatcher.dispatch(&mut world);
		world.maintain();
	});
	Ok(())
}
