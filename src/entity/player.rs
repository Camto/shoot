use macroquad::prelude::*;
use crate::window;
use crate::collision;
use crate::float_utils;
use crate::entity;
use crate::entity::Entity;
use crate::entity::circle::Circle;


const hp_max: usize = 4;
const player_speed: f32 = 400.0;

pub struct Player {
	pub tex_id: usize,
	pub dmg_tex_ids: [usize; 3],
	pub body: Circle,
	hp: usize,
	was_killed: bool
}

pub struct Player_Options {
	pub tex_id: usize,
	pub dmg_tex_ids: [usize; 3],
	pub body: Circle
}

impl Player {
	pub fn new(init: Player_Options) -> Self {
		Player {
			tex_id: init.tex_id,
			dmg_tex_ids: init.dmg_tex_ids,
			body: init.body,
			hp: hp_max,
			was_killed: false
		}
	}
}

impl Entity for Player {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		let speed: f32 =
			player_speed *
				if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
					0.5
				} else {
					1.0
				};
		
		if is_key_down(KeyCode::Up) {
			self.body.y -= speed * tf;
		}
		if is_key_down(KeyCode::Down) {
			self.body.y += speed * tf;
		}
		if is_key_down(KeyCode::Left) {
			self.body.x -= speed * tf;
		}
		if is_key_down(KeyCode::Right) {
			self.body.x += speed * tf;
		}
		
		if self.body.x < self.body.r {
			self.body.x = self.body.r
		}
		if self.body.y < self.body.r {
			self.body.y = self.body.r
		}
		if self.body.x > window::width - self.body.r {
			self.body.x = window::width - self.body.r
		}
		if self.body.y > window::height - self.body.r {
			self.body.y = window::height - self.body.r
		}
		
		let cam_x_off = float_utils::lerp(self.body.x, self.body.r, window::width - self.body.r, -25.0, 25.0);
		let cam_y_off = float_utils::lerp(self.body.y, self.body.r, window::height - self.body.r, -25.0, 25.0);
		set_camera(&Camera3D {
			position: vec3(window::width/2.0, window::height/2.0, -500.0),
			target: vec3(window::width/2.0 + cam_x_off, window::height/2.0 + cam_y_off, 0.0),
			up: vec3(0.0, -1.0, 0.0),
			..Default::default()
		});
		
		Default::default()
	}
	
	fn render(&self, texs: &entity::Textures) {
		let tex: &Texture2D = &texs[self.tex_id];
		
		//self.body.render(texs);
		
		draw_cube(
			vec3(self.body.x, self.body.y, 0.0),
			vec3(tex.width(), tex.height(), 0.0),
			Some(tex), WHITE
		);
		
		if self.hp < hp_max {
			let dmg_tex: &Texture2D = &texs[self.dmg_tex_ids[hp_max - self.hp - 1]];
			draw_cube(
				vec3(self.body.x, self.body.y, -1.0),
				vec3(dmg_tex.width(), dmg_tex.height(), 0.0),
				Some(dmg_tex), WHITE
			);
		}
	}
	
	fn collided_with(&mut self, collision_id: usize) {
		if collision_id == collision::enemy_coll_id {
			self.hp -= 1;
			if self.hp <= 0 {
				self.was_killed = true;
			}
		}
	}
	
	fn is_dead(&self) -> bool {
		self.was_killed
	}
	
	fn get_collision_id(&self) -> usize {
		collision::player_coll_id
	}
	
	fn checks_collision_with(&self) -> &'static [usize] {
		collision::collide_with_enemies
	}
	
	fn get_hitbox(&self) -> Circle {
		self.body
	}
}