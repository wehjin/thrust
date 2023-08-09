pub mod three;
pub mod dom;
pub mod basics;
pub mod four;
pub mod app;

use specs::{Builder, DispatcherBuilder, World, WorldExt};
use dom::Window;
use crate::app::components::now_in_seconds::NowInSeconds;
use crate::app::components::spin_target::SpinTarget;
use crate::app::systems::spin_system::SpinSystem;
use crate::dom::{Error};
use crate::four::render_system::RenderSystem;

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");
	let renderer = four::Renderer::new(&Window::connect());
	let mut world = World::new();
	world.register::<SpinTarget>();
	world.insert(NowInSeconds(0.));
	world.insert(renderer.clone());
	init_world(&mut world, renderer.scene());

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

fn init_world(world: &mut World, scene: &three::Scene) {
	add_cube(world, scene);

	const HEIGHT: f64 = 0.050;
	let mesh = {
		let geometry = three::CylinderGeometry::new(1.5, 1.5, HEIGHT);
		let material = three::MeshBasicMaterial::new();
		material.color().set(0.47, 0.53, 0.60);
		three::Mesh::new(geometry.as_ref(), material.as_ref())
	};
	mesh.position().set_y(-HEIGHT / 2.);
	scene.add(&mesh);
}

fn add_cube(world: &mut World, scene: &three::Scene) {
	let mesh = {
		let geometry = three::BoxGeometry::new(2., 1., 1.);
		let material = three::MeshBasicMaterial::new();
		material.color().set(0., 0., 1.);
		three::Mesh::new(geometry.as_ref(), material.as_ref())
	};
	mesh.position().set_xyz(0., 1.6, -3.);
	mesh.rotation().set(0., 0., 0.);
	scene.add(&mesh);
	world.create_entity().with(SpinTarget { euler: mesh.rotation() }).build();
}
