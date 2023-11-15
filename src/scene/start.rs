use macroquad::prelude::*;
use crate::window;
use crate::entity;
use crate::entity::text::Text;
use crate::entity::game_starter::Game_Starter;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


pub struct Start {}

impl Scene for Start {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		set_default_camera();
		
		let mut entities: scene::Entities = [
			vec![
				Box::new(Game_Starter {})
			],
			// In this layer because Z issues I will not properly fix.
			vec![
				Box::new(Text {
					x: window::mid_width, y: window::mid_height - 60.0,
					text: "Shoot, the Game".to_string(),
					font_size: 60
				}),
				Box::new(Text {
					x: window::mid_width, y: window::mid_height + 40.0,
					text: "Press enter to start".to_string(),
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