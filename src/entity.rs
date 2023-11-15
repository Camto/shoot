pub mod circle;
pub mod background;
pub mod player;
pub mod guy;
pub mod pew;
pub mod text;
pub mod game_starter;
pub mod win_checker;

use macroquad::{audio, prelude::*};
use crate::collision;
use crate::entity::circle::Circle;
use crate::scene::Scene;


pub type Textures = Vec<Texture2D>;
pub type Sounds = Vec<audio::Sound>;

#[allow(unused_variables)]
pub trait Entity {
	fn update(&mut self, tf: f32) -> Update_Result {
		Default::default()
	}
	
	fn render(&self, texs: &Textures, sounds: &Sounds, font: &Font) {}
	
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
	pub new_entities: Vec<Box<dyn Entity>>,
	pub change_scene: Option<Box<dyn Scene>>
}

impl Default for Update_Result {
	fn default() -> Self {
		Update_Result {
			new_entities: vec![],
			change_scene: None
		}
	}
}