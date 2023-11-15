use crate::entity;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


pub struct Lose {}

impl Scene for Lose {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		let mut entities = (scene::Empty_Scene {}).init(&texs);
		background::add(&mut entities, texs);
		entities
	}
}