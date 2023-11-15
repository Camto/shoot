use macroquad::prelude::*;
use crate::collision;
use crate::entity;
use crate::entity::Entity;
use crate::entity::circle::Circle;


const frame_len: f32 = 0.5;
const anim_len: usize = 2;

const player_anim_texs: [usize; anim_len] = [5, 6];
const enemy_anim_texs: [usize; anim_len] = [8, 9];

pub struct Pew {
	pub is_friendly: bool,
	pub body: Circle,
	pub xv: f32, pub yv: f32,
	
	anim_sprite: usize,
	anim_timer: f32,
	
	was_killed: bool
}

pub struct Pew_Options {
	pub is_friendly: bool,
	pub body: Circle,
	pub xv: f32, pub yv: f32
}

impl Default for Pew_Options {
	fn default() -> Self {
		Pew_Options {
			is_friendly: false,
			body: Circle { x: 100.0, y: 50.0, r: 15.0 },
			xv: -200.0, yv: 0.0
		}
	}
}

impl Pew {
	pub fn new(init: Pew_Options) -> Self {
		Pew {
			is_friendly: init.is_friendly,
			body: init.body,
			xv: init.xv, yv: init.yv,
			
			anim_sprite: 0,
			anim_timer: 0.0,
			
			was_killed: false
		}
	}
}

impl Entity for Pew {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		self.body.x += self.xv * tf;
		self.body.y += self.yv * tf;
		
		self.anim_timer += tf;
		if self.anim_timer > frame_len {
			self.anim_timer = 0.0;
			self.anim_sprite += 1;
			if self.anim_sprite >= anim_len {
				self.anim_sprite = 0;
			}
		}
		
		Default::default()
	}
	
	fn render(&self, texs: &entity::Textures) {
		let anim_texs: [usize; anim_len] =
			if self.is_friendly { player_anim_texs }
			else { enemy_anim_texs };
		let tex: &Texture2D = &texs[anim_texs[self.anim_sprite]];
		
		//self.body.render(texs);
		
		draw_cube(
			vec3(self.body.x, self.body.y, 0.0),
			vec3(tex.width(), tex.height(), 0.0),
			Some(tex), WHITE
		);
	}
	
	// Killed if it collides with anything that was checking for it.
	fn collided_with(&mut self, _: usize) {
		self.was_killed = true;
	}
	
	fn is_dead(&self) -> bool {
		self.was_killed || self.body.off_screen()
	}
	
	fn get_collision_id(&self) -> usize {
		if self.is_friendly {
			collision::bullet_id
		} else {
			collision::enemy_id
		}
	}
	
	fn get_hitbox(&self) -> Circle {
		self.body
	}
}