use macroquad::prelude::*;
use crate::entity;
use crate::entity::Entity;


pub struct Text {
	pub x: f32, pub y: f32,
	pub text: String,
	pub font_size: u16
}

impl Entity for Text {
	fn render(&self, _: &entity::Textures, _: &entity::Sounds, font: &Font) {
		let center = get_text_center(&self.text, Some(font), self.font_size, 1.0, 0.0);
		draw_text_ex(
			&self.text,
			self.x - center.x,
			self.y - center.y,
			TextParams {
				font: Some(font),
				font_size: self.font_size,
				..Default::default()
			},
		);
	}
}