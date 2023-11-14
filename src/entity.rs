pub mod circle;
pub mod player;
pub mod guy;
pub mod pew;

use crate::entity::circle::Circle;


pub const no_coll_checks: &'static [usize] = &[];

pub trait Entity {
	fn update(&mut self, _: f32) -> Update_Result {
		Default::default()
	}
	
	fn render(&self) {}
	
	fn is_dead(&self) -> bool { false }
	
	fn get_collision_id(&self) -> usize { 0 }
	
	fn checks_collision_with(&self) -> &'static [usize] {
		no_coll_checks
	}
	
	fn get_hitbox(&self) -> Circle {
		Default::default()
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