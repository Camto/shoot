use macroquad::prelude::*;
use crate::window;
use crate::lerp;
use crate::entity;
use crate::entity::circle::Circle;


pub const player_coll_id: i32 = 1;
const player_speed: f32 = 300.0;

pub struct Player {
	pub body: Circle
}

impl entity::Entity for Player {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		let speed: f32 =
			if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
				player_speed * 0.5
			} else {
				player_speed
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
		
		if self.body.x < -100.0 - self.body.r {
			self.body.x = -100.0 - self.body.r
		}
		if self.body.y < -50.0 - self.body.r {
			self.body.y = -50.0 - self.body.r
		}
		if self.body.x > window::window_width + 100.0 + self.body.r {
			self.body.x = window::window_width + 100.0 + self.body.r
		}
		if self.body.y > window::window_height + 50.0 + self.body.r {
			self.body.y = window::window_height + 50.0 + self.body.r
		}
		
		let cam_x_off = lerp::lerp(self.body.x, 0.0, window::window_width, -50.0, 50.0);
		let cam_y_off = lerp::lerp(self.body.y, 0.0, window::window_height, -50.0, 50.0);
		set_camera(&Camera3D {
			position: vec3(window::window_width/2.0 - cam_x_off, window::window_height/2.0 - cam_y_off, -600.0),
			target: vec3(window::window_width/2.0, window::window_height/2.0, 0.0),
			up: vec3(0.0, -1.0, 0.0),
			..Default::default()
		});
		
		Default::default()
	}
	
	fn render(&self) {
		self.body.render()
	}
	
	fn get_collision_id(&self) -> i32 {
		player_coll_id
	}
	
	fn get_hitbox(&self) -> Circle {
		self.body
	}
}