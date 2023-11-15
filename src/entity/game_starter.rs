use macroquad::prelude::*;
use crate::entity;
use crate::entity::Entity;
use crate::scene::level::Level;


pub struct Game_Starter {}

impl Entity for Game_Starter {
	fn update(&mut self, _: f32) -> entity::Update_Result {
		if is_key_down(KeyCode::Enter) {
			entity::Update_Result {
				change_scene: Some(Box::new(Level {})),
				..Default::default()
			}
		} else {
			Default::default()
		}
	}
}