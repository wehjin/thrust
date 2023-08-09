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
