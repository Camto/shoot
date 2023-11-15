use macroquad::prelude::*;
use crate::window;
use crate::collision;
use crate::entity;
use crate::entity::Entity;
use crate::entity::circle::Circle;
use crate::scene::win::Win;


const hitbox: Circle = Circle {
	x: window::width * 0.5, y: window::height * 0.5,
	r: 5000.0
};

pub struct Win_Checker {
	enemies_touched: u32
}

impl Win_Checker {
	pub fn new() -> Self {
		Win_Checker {
			enemies_touched: 1
		}
	}
}

impl Entity for Win_Checker {
	fn update(&mut self, _: f32) -> entity::Update_Result {
		if self.enemies_touched > 0 {
			self.enemies_touched = 0;
			Default::default()
		} else {
			entity::Update_Result {
				change_scene: Some(Box::new(Win {})),
				..Default::default()
			}
		}
	}
	
	fn collided_with(&mut self, collision_id: usize) {
		if collision_id == collision::enemy_id {
			self.enemies_touched += 1;
		}
	}
	
	fn checks_collision_with(&self) -> &'static [usize] {
		collision::with_enemies
	}
	
	fn get_hitbox(&self) -> Circle {
		hitbox
	}
}