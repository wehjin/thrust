pub mod three;
pub mod dom;
pub mod basics;
pub mod four;
pub mod app;

use specs::{Builder, Component, DispatcherBuilder, Join, Read, ReadStorage, System, VecStorage, World, WorldExt};
use dom::Window;
use crate::dom::{console, Error};
use crate::four::render_system::RenderSystem;

#[derive(Default)]
struct ActiveColor(f64, f64, f64);

impl ActiveColor {
	fn to_rgb_triple(&self) -> (f64, f64, f64) {
		(self.0, self.1, self.2)
	}
	fn from_index(color_idx: usize) -> Self {
		let (r, g, b) = match color_idx % 3 {
			0 => (0.0, 0.0, 1.0),
			1 => (1.0, 0.0, 0.0),
			_ => (0.0, 1.0, 0.0),
		};
		ActiveColor(r, g, b)
	}
}

struct Cube {
	color: three::Color,
}

impl Component for Cube {
	type Storage = VecStorage<Self>;
}


struct UpdateCubeColor;

impl<'a> System<'a> for UpdateCubeColor {
	type SystemData = (Read<'a, ActiveColor>, ReadStorage<'a, Cube>);

	fn run(&mut self, (active_color, cube): Self::SystemData) {
		let (r, g, b) = active_color.to_rgb_triple();
		for cube in cube.join() {
			cube.color.set(r, g, b);
		}
	}
}

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");
	let window = Window::connect();
	let renderer = four::Renderer::new(&window);

	let mut world = World::new();
	world.register::<Cube>();
	let color = init_world(renderer.scene());
	let mut color_idx = 0;
	world.insert(ActiveColor::from_index(color_idx));
	world.create_entity().with(Cube { color }).build();
	world.insert(renderer.clone());
	dispatch_world(&mut world);

	window.add_and_forget_keydown_listener(move |event: web_sys::KeyboardEvent| {
		let key = &event.key();
		console::log(&format!("keypress: {}", key));
		if key == "c" {
			color_idx += 1;
			{
				let mut write_color = world.write_resource::<ActiveColor>();
				*write_color = ActiveColor::from_index(color_idx);
			}
			dispatch_world(&mut world);
		}
	});
	Ok(())
}

fn dispatch_world(mut world: &mut World) {
	let mut dispatcher = DispatcherBuilder::new()
		.with(UpdateCubeColor, "update_cube_color", &[])
		.with_thread_local(RenderSystem)
		.build();
	dispatcher.dispatch(&mut world);
	world.maintain();
}

fn init_world(scene: &three::Scene) -> three::Color {
	let color = add_cube(scene);

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

fn add_cube(scene: &three::Scene) -> three::Color {
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
	color
}