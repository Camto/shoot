use macroquad::{audio, prelude::*};
use crate::window;
use crate::collision;
use crate::float_utils;
use crate::entity;
use crate::entity::Entity;
use crate::entity::circle::Circle;
use crate::entity::pew;
use crate::entity::pew::Pew;
use crate::scene::lose::Lose;


const hp_max: usize = 4;
const speed: f32 = 400.0;
const bullet_speed: f32 = 800.0;
const shoot_cycle: f32 = 0.5;

const tex_id: usize = 1;
const damage_tex_ids: [usize; hp_max - 1] = [2, 3, 4];

const shoot_sfx_id: usize = 0;
const damage_sfx_id: usize = 2;

pub struct Player {
	pub body: Circle,
	hp: usize,
	just_damaged: bool,
	shoot_timer: f32,
	just_shot: bool
}

pub struct Player_Options {
	pub body: Circle
}

impl Player {
	pub fn new(init: Player_Options) -> Self {
		Player {
			body: init.body,
			hp: hp_max,
			just_damaged: false,
			shoot_timer: 0.0,
			just_shot: false
		}
	}
}

impl Entity for Player {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		self.just_damaged = false;
		
		if self.hp <= 0 {
			return entity::Update_Result {
				change_scene: Some(Box::new(Lose {})),
				..Default::default()
			}
		}
		
		let curr_speed: f32 =
			speed *
				if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
					0.5
				} else {
					1.0
				};
		
		if is_key_down(KeyCode::Up) {
			self.body.y -= curr_speed * tf;
		}
		if is_key_down(KeyCode::Down) {
			self.body.y += curr_speed * tf;
		}
		if is_key_down(KeyCode::Left) {
			self.body.x -= curr_speed * tf;
		}
		if is_key_down(KeyCode::Right) {
			self.body.x += curr_speed * tf;
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
		
		self.shoot_timer += tf;
		if self.shoot_timer >= shoot_cycle {
			self.shoot_timer = 0.0;
			self.just_shot = true;
			
			entity::Update_Result {
				new_entities: vec![
					Box::new(Pew::new(pew::Pew_Options {
						is_friendly: true,
						body: Circle { x: self.body.x + 30.0, y: self.body.y, r: 15.0 },
						xv: bullet_speed,
						..Default::default()
					}))
				],
				..Default::default()
			}
		} else {
			self.just_shot = false;
			Default::default()
		}
	}
	
	fn render(&self, texs: &entity::Textures, sounds: &entity::Sounds, _: &Font) {
		let tex: &Texture2D = &texs[tex_id];
		draw_cube(
			vec3(self.body.x, self.body.y, 0.0),
			vec3(tex.width(), tex.height(), 0.0),
			Some(tex), WHITE
		);
		
		if self.hp > 0 && self.hp < hp_max {
			let dmg_tex: &Texture2D = &texs[damage_tex_ids[hp_max - self.hp - 1]];
			draw_cube(
				vec3(self.body.x, self.body.y, -1.0),
				vec3(dmg_tex.width(), dmg_tex.height(), 0.0),
				Some(dmg_tex), WHITE
			);
		}
		
		if self.just_shot {
			let shoot_sfx: &audio::Sound = &sounds[shoot_sfx_id];
			audio::play_sound_once(&shoot_sfx);
		}
		
		if self.just_damaged {
			let damage_sfx: &audio::Sound = &sounds[damage_sfx_id];
			audio::play_sound_once(&damage_sfx);
		}
	}
	
	fn collided_with(&mut self, collision_id: usize) {
		if collision_id == collision::enemy_bullet_id && self.hp > 0 {
			self.hp -= 1;
			self.just_damaged = true;
		}
	}
	
	fn get_collision_id(&self) -> usize {
		collision::player_id
	}
	
	fn checks_collision_with(&self) -> &'static [usize] {
		collision::with_enemy_bullets
	}
	
	fn get_hitbox(&self) -> Circle {
		self.body
	}
}