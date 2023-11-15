use macroquad::prelude::*;
use crate::window;
use crate::entity;
use crate::entity::text::Text;
use crate::entity::sound_once;
use crate::entity::sound_once::Sound_Once;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


const sfx_id: usize = 3;

pub struct Win {}

impl Scene for Win {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		set_default_camera();
		
		let mut entities: scene::Entities = [
			vec![
				Box::new(Sound_Once::new(sound_once::Sound_Once_Options { sfx_id }))
			],
			// In this layer because Z issues I will not properly fix.
			vec![
				Box::new(Text {
					x: window::mid_width, y: window::mid_height,
					text: "You win!".to_string(),
					font_size: 60
				})
			],
			vec![],
			vec![],
			vec![]
		];
		background::add(&mut entities, 0.0, texs);
		entities
	}
}