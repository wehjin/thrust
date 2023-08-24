pub mod three;
pub mod dom;
pub mod basics;
pub mod four;
pub mod app;

use specs::{Builder, DispatcherBuilder, World, WorldExt};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use dom::Window;
use crate::app::components::now_in_seconds::NowInSeconds;
use crate::app::components::spin_target::SpinTarget;
use crate::app::systems::spin_system::SpinSystem;
use crate::dom::{console, Error};
use crate::four::render_system::RenderSystem;

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");
	let window = Window::connect();
	let renderer = four::Renderer::new(&window);

	let mut world = World::new();
	world.register::<SpinTarget>();
	world.insert(NowInSeconds(0.));
	world.insert(renderer.clone());
	let color = init_world(&mut world, renderer.scene());
	{
		let element = window.document().body().expect("must have body");
		{
			let mut color_idx = 0;
			let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
				let key = &event.key();
				console::log(&format!("keypress: {}", key));
				if key == "c" {
					color_idx += 1;
					let (r, g, b) = match color_idx % 3 {
						0 => (0.0, 0.0, 1.0),
						1 => (1.0, 0.0, 0.0),
						_ => (0.0, 1.0, 0.0),
					};
					color.set(r, g, b);
				}
			});
			element.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
			closure.forget();
		}
	}

	let mut dispatcher = DispatcherBuilder::new()
		.with(SpinSystem, "sys_spin", &[])
		.with_thread_local(RenderSystem)
		.build();

	renderer.set_animation_loop_with_forget(move |time| {
		*world.write_resource::<NowInSeconds>() = NowInSeconds(time / 1000.);
		dispatcher.dispatch(&mut world);
		world.maintain();
	});
	Ok(())
}

fn init_world(world: &mut World, scene: &three::Scene) -> three::Color {
	let color = add_cube(world, scene);

	const HEIGHT: f64 = 0.050;
	let mesh = {
		let geometry = three::CylinderGeometry::new(1.5, 1.5, HEIGHT);
		let material = three::MeshBasicMaterial::new();
		material.color().set(0.47, 0.53, 0.60);
		three::Mesh::new(geometry.as_ref(), material.as_ref())
	};
	mesh.position().set_y(-HEIGHT / 2.);
	scene.add(&mesh);
	color
}

fn add_cube(world: &mut World, scene: &three::Scene) -> three::Color {
	let (mesh, color) = {
		let geometry = three::BoxGeometry::new(2., 1., 1.);
		let material = three::MeshBasicMaterial::new();
		material.color().set(0., 0., 1.);
		let mesh = three::Mesh::new(geometry.as_ref(), material.as_ref());
		(mesh, material.color())
	};
	mesh.position().set_xyz(0., 1.6, -3.);
	mesh.rotation().set(0., 0., 0.);
	scene.add(&mesh);
	world.create_entity().with(SpinTarget { euler: mesh.rotation() }).build();
	color
}
