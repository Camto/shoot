use crate::entity;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


pub struct Win {}

impl Scene for Win {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		let mut entities = (scene::Empty_Scene {}).init(&texs);
		background::add(&mut entities, 0.0, texs);
		entities
	}
}