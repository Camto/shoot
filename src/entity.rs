pub mod circle;
pub mod background;
pub mod player;
pub mod guy;
pub mod pew;

use macroquad::prelude::*;
use crate::collision;
use crate::entity::circle::Circle;


pub type Textures = Vec<Texture2D>;

pub trait Entity {
	fn update(&mut self, _: f32) -> Update_Result {
		Default::default()
	}
	
	fn render(&self, texs: &Textures) {}
	
	#[allow(unused_variables)]
	fn collided_with(&mut self, collision_id: usize) {}
	
	fn is_dead(&self) -> bool { false }
	
	fn get_collision_id(&self) -> usize { 0 }
	
	fn checks_collision_with(&self) -> &'static [usize] {
		collision::no_checks
	}
	
	fn get_hitbox(&self) -> Circle {
		Default::default()
	}
	
	fn collides_with(&self, other: &dyn Entity) -> bool {
		self.get_hitbox().collides_with(other)
	}
}

pub struct Update_Result {
	pub new_entities: Vec<Box<dyn Entity>>
}

impl Default for Update_Result {
	fn default() -> Self {
		Update_Result {
			new_entities: vec![]
		}
	}
}