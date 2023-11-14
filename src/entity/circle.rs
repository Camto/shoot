use macroquad::prelude::*;
use crate::window;
use crate::entity;

#[derive(Copy, Clone)]
pub struct Circle {
	pub x: f32, pub y: f32,
	pub r: f32
}

impl entity::Entity for Circle {
	fn get_hitbox(&self) -> Circle {
		self.clone()
	}
	
	fn render(&self) {
		draw_circle(self.x, self.y, self.r, YELLOW);
	}
}

impl Circle {
	pub fn dist(&self, other: &Self) -> f32 {
		let x_dist = other.x - self.x;
		let y_dist = other.y - self.y;
		(x_dist * x_dist + y_dist * y_dist).sqrt()
	}
	
	pub fn does_collide(&self, other: &Self) -> bool {
		self.dist(other) < self.r + other.r
	}
	
	pub fn off_screen(&self) -> bool {
		self.x < -500.0 - self.r ||
			self.y < -500.0 - self.r ||
			self.x > window::window_width + 500.0 + self.r ||
			self.y > window::window_height + 500.0 + self.r
	}
}

impl Default for Circle {
	fn default() -> Self {
		Circle { x: 0.0, y: 0.0, r: 15.0 }
	}
}