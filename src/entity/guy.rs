use macroquad::prelude::*;
use crate::collision;
use crate::float_utils;
use crate::entity;
use crate::entity::Entity;
use crate::entity::circle::Circle;
use crate::Pew;


const guy_speed: f32 = 150.0;

pub struct Guy {
	pub tex_id: usize,
	pub body: Circle,
	
	pub path: Vec<(f32, f32)>,
	dists: Vec<f32>,
	path_idx: usize,
	frw_through_path: bool,
	move_timer: f32,
	
	started_path: bool,
	starting_x: f32,
	starting_y: f32, 
	
	pub shoot_cycle: f32,
	pub shoot_timer: f32
}

pub struct Guy_Options {
	pub tex_id: usize,
	pub body: Circle,
	pub path: Vec<(f32, f32)>,
	pub shoot_cycle: f32,
	pub shoot_timer: f32
}

impl Guy {
	pub fn new(init: Guy_Options) -> Self {
		Guy {
			started_path: false,
			starting_x: init.body.x,
			starting_y: init.body.y,
			
			tex_id: init.tex_id,
			body: init.body,
			
			dists:
				init.path.windows(2)
					.map(|slice| {
						let (x1, y1) = slice[0];
						let (x2, y2) = slice[1];
						float_utils::dist(x1, y1, x2, y2)
					})
					.collect(),
			path: init.path,
			path_idx: 0,
			frw_through_path: true,
			move_timer: 0.0,
			
			shoot_cycle: init.shoot_cycle,
			shoot_timer: 0.0
		}
	}
}

impl Default for Guy_Options {
	fn default() -> Self {
		Guy_Options {
			tex_id: 0,
			body: Circle { x: 100.0, y: 50.0, r: 30.0 },
			path: vec![(100.0, 50.0)],
			shoot_cycle: 2.5,
			shoot_timer: 2.0
		}
	}
}

impl Entity for Guy {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		if self.started_path {
			if self.path.len() > 1 {
				let dist = self.dists[self.path_idx - if self.frw_through_path { 0 } else { 1 }];
				
				self.move_timer += guy_speed * tf;
				if self.move_timer > dist {
					self.move_timer -= dist;
					if self.frw_through_path {
						self.path_idx += 1;
						if self.path_idx == self.path.len() - 1 {
							self.frw_through_path = false;
						}
					} else {
						self.path_idx -= 1;
						if self.path_idx == 0 {
							self.frw_through_path = true;
						}
					}
				}
				
				let eased = float_utils::quad_ease(float_utils::lerp(self.move_timer, 0.0, dist, 0.0, 1.0));
				let (x1, y1) = self.path[self.path_idx];
				let (x2, y2) = self.path[if self.frw_through_path { self.path_idx + 1 } else { self.path_idx - 1 }];
				self.body.x = float_utils::lerp(eased, 0.0, 1.0, x1, x2);
				self.body.y = float_utils::lerp(eased, 0.0, 1.0, y1, y2);
			}
			
			self.shoot_timer += tf;
			if self.shoot_timer >= self.shoot_cycle {
				self.shoot_timer = 0.0;
				
				entity::Update_Result {
					new_entities: vec![
						Box::new(Pew {
							body: Circle { x: self.body.x - 20.0, y: self.body.y - 20.0, r: 10.0 },
							yv: -40.0,
							..Default::default()
						}),
						Box::new(Pew {
							body: Circle { x: self.body.x - 30.0, y: self.body.y, r: 10.0 },
							..Default::default()
						}),
						Box::new(Pew {
							body: Circle { x: self.body.x - 20.0, y: self.body.y + 20.0, r: 10.0 },
							yv: 40.0,
							..Default::default()
						})
					]
				}
			} else {
				Default::default()
			}
		} else {
			let (tx, ty) = self.path[0];
			let dist = float_utils::dist(self.starting_x, self.starting_y, tx, ty);
			
			self.move_timer += 2.0 * guy_speed * tf;
			if self.move_timer > dist {
				self.move_timer = 0.0;
				self.started_path = true;
			}
			
			let eased = float_utils::quad_ease(float_utils::lerp(self.move_timer, 0.0, dist, 0.0, 1.0));
			self.body.x = float_utils::lerp(eased, 0.0, 1.0, self.starting_x, tx);
			self.body.y = float_utils::lerp(eased, 0.0, 1.0, self.starting_y, ty);
			
			Default::default()
		}
	}
	
	fn render(&self, texs: &entity::Textures) {
		let tex: &Texture2D = &texs[self.tex_id];
		
		//self.body.render(texs);
		
		draw_cube(
			vec3(self.body.x, self.body.y, 0.0),
			vec3(tex.width(), tex.height(), 0.0),
			Some(tex), WHITE
		);
	}
	
	fn get_collision_id(&self) -> usize {
		collision::generic_coll_id
	}
	
	fn get_hitbox(&self) -> Circle {
		self.body
	}
}