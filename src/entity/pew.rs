use macroquad::prelude::*;
use crate::entity;
use crate::circle;


pub const pew_coll_id: i32 = 3;

pub struct Pew {
	pub body: circle::Circle,
	pub xv: f32, pub yv: f32
}

impl Default for Pew {
	fn default() -> Self {
		Pew {
			body: circle::Circle { x: 100.0, y: 50.0, r: 10.0 },
			xv: -200.0, yv: 0.0
		}
	}
}

impl entity::Entity for Pew {
	fn get_collision_id(&self) -> i32 {
		pew_coll_id
	}
	
	fn get_hitbox(&self) -> circle::Circle {
		self.body
	}
	
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		self.body.x += self.xv * tf;
		self.body.y += self.yv * tf;
		
		entity::Update_Result {
			kill: self.body.off_screen(),
			..Default::default()
		}
	}
	
	fn render(&self) {
		self.body.render();
	}
}