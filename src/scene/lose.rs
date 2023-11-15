use macroquad::prelude::*;
use crate::window;
use crate::entity;
use crate::entity::text::Text;
use crate::entity::sound_once;
use crate::entity::sound_once::Sound_Once;
use crate::entity::game_starter::Game_Starter;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


const sfx_id: usize = 4;

pub struct Lose {}

impl Scene for Lose {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		set_default_camera();
		
		let mut entities: scene::Entities = [
			vec![
				Box::new(Game_Starter {}),
				Box::new(Sound_Once::new(sound_once::Sound_Once_Options { sfx_id }))
			],
			// In this layer because Z issues I will not properly fix.
			vec![
				Box::new(Text {
					x: window::mid_width, y: window::mid_height - 40.0,
					text: "You lose!".to_string(),
					font_size: 40
				}),
				Box::new(Text {
					x: window::mid_width, y: window::mid_height + 40.0,
					text: "Press enter to restart".to_string(),
					font_size: 40
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