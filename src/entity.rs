pub mod player;
pub mod guy;
pub mod pew;

use crate::circle;


pub trait Entity {
	fn get_collision_id(&self) -> i32 { 0 }
	
	fn get_hitbox(&self) -> circle::Circle {
		Default::default()
	}
	
	fn update(&mut self, _: f32) -> Update_Result {
		Default::default()
	}
	
	fn render(&self) {}
}

pub struct Update_Result {
	pub kill: bool,
	pub new_entities: Vec<Box<dyn Entity>>
}

impl Default for Update_Result {
	fn default() -> Self {
		Update_Result {
			kill: false,
			new_entities: vec![]
		}
	}
}