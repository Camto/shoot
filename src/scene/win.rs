use macroquad::prelude::*;
use crate::window;
use crate::entity;
use crate::entity::text::Text;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


pub struct Win {}

impl Scene for Win {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		set_default_camera();
		
		let mut entities: scene::Entities = [
			vec![],
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