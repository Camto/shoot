use macroquad::{audio, prelude::*};
use crate::entity;
use crate::entity::Entity;


pub struct Sound_Once {
	pub sfx_id: usize,
	timer: u8
}

pub struct Sound_Once_Options {
	pub sfx_id: usize
}

impl Sound_Once {
	pub fn new(init: Sound_Once_Options) -> Self {
		Sound_Once {
			sfx_id: init.sfx_id,
			timer: 2
		}
	}
}

impl Entity for Sound_Once {
	fn update(&mut self, _: f32) -> entity::Update_Result {
		self.timer -= 1;
		Default::default()
	}
	
	fn render(&self, _: &entity::Textures, sounds: &entity::Sounds, _: &Font) {
		let sfx: &audio::Sound = &sounds[self.sfx_id];
		audio::play_sound_once(&sfx);
	}
	
	fn is_dead(&self) -> bool {
		self.timer <= 0
	}
}