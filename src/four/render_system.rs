use specs::{Read, System};
use crate::four::renderer::Renderer;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
	type SystemData = Read<'a, Renderer>;

	fn run(&mut self, renderer: Self::SystemData) {
		renderer.as_three().render(renderer.scene(), renderer.camera());
	}
}
