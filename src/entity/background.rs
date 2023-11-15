use macroquad::prelude::*;
use crate::window;
use crate::entity;
use crate::entity::Entity;


pub struct Background {
	pub tex_id: usize,
	pub tex_width: f32,
	pub offset: f32,
	pub scroll_speed: f32
}

impl Entity for Background {
	fn update(&mut self, tf: f32) -> entity::Update_Result {
		self.offset -= self.scroll_speed * tf;
		if self.offset < -2.0 * self.tex_width {
			self.offset += 4.0 * self.tex_width
		}
		
		Default::default()
	}
	
	fn render(&self, texs: &entity::Textures) {
		let tex: &Texture2D = &texs[self.tex_id];
		draw_cube(
			vec3(window::width * 0.5 + self.offset, window::height * 0.5, 50.0),
			vec3(tex.width(), tex.height(), 1.0),
			Some(tex), WHITE
		);
	}
}