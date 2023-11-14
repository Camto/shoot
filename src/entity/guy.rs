use macroquad::prelude::*;
use crate::collision;
use crate::entity;
use crate::entity::Entity;
use crate::entity::circle::Circle;
use crate::Pew;


pub struct Guy {
	pub body: Circle,
	
	pub shoot_cycle: f32,
	pub shoot_timer: f32
}

impl Default for Guy {
	fn default() -> Self {
		Guy {
			body: Circle { x: 100.0, y: 50.0, r: 30.0 },
			shoot_cycle: 2.5,
			shoot_timer: 2.0
		}
	}
}

impl Entity for Guy {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		self.body.x -= 5.0 * tf;
		
		self.shoot_timer += tf;
		if self.shoot_timer >= self.shoot_cycle {
			self.shoot_timer = 0.0;
			
			entity::Update_Result {
				new_entities: vec![
					Box::new(Pew {
						body: Circle { x: self.body.x, y: self.body.y - 20.0, r: 10.0 },
						yv: -30.0,
						..Default::default()
					}),
					Box::new(Pew {
						body: Circle { x: self.body.x, y: self.body.y, r: 10.0 },
						..Default::default()
					}),
					Box::new(Pew {
						body: Circle { x: self.body.x, y: self.body.y + 20.0, r: 10.0 },
						yv: 30.0,
						..Default::default()
					})
				]
			}
		} else {
			Default::default()
		}
	}
	
	fn render(&self) {
		self.body.render();
	}
	
	fn get_collision_id(&self) -> usize {
		collision::generic_coll_id
	}
	
	fn get_hitbox(&self) -> Circle {
		self.body
	}
}