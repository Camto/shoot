use macroquad::prelude::*;
use crate::window;
use crate::float_utils;
use crate::entity;
use crate::entity::Entity;


#[derive(Copy, Clone)]
pub struct Circle {
	pub x: f32, pub y: f32,
	pub r: f32
}

impl Entity for Circle {
	fn get_hitbox(&self) -> Circle {
		*self
	}
	
	fn render(&self, texs: &entity::Textures) {
		draw_circle(self.x, self.y, self.r, YELLOW);
	}
	
	fn collides_with(&self, other: &dyn Entity) -> bool {
		let other_circle = other.get_hitbox();
		self.center_dist(&other_circle) < self.r + other_circle.r
	}
}

impl Circle {
	pub fn center_dist(&self, other: &Self) -> f32 {
		float_utils::dist(self.x, self.y, other.x, other.y)
	}
	
	pub fn off_screen(&self) -> bool {
		self.x < -500.0 - self.r ||
			self.y < -500.0 - self.r ||
			self.x > window::width + 500.0 + self.r ||
			self.y > window::height + 500.0 + self.r
	}
}

impl Default for Circle {
	fn default() -> Self {
		Circle { x: 0.0, y: 0.0, r: 15.0 }
	}
}