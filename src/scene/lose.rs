use macroquad::prelude::*;
use crate::window;
use crate::entity;
use crate::entity::text::Text;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


pub struct Lose {}

impl Scene for Lose {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		set_default_camera();
		
		let mut entities: scene::Entities = [
			vec![],
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