use macroquad::prelude::*;
use crate::entity;
use crate::entity::Entity;


pub struct Text {
	pub x: f32, pub y: f32,
	text: String
}

impl Entity for Text {
	fn render(&self, texs: &entity::Textures, sounds: &entity::Sounds, font: &Font) {
		
	}
}