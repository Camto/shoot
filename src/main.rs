#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]


pub mod window;
pub mod lerp;
pub mod entity;

use macroquad::prelude::*;
use crate::entity::Entity;
use crate::entity::circle::Circle;
use crate::entity::player::Player;
use crate::entity::guy::Guy;
use crate::entity::pew::Pew;


#[macroquad::main("Shoot")]
async fn main() {
	request_new_screen_size(window::window_width, window::window_height);
	next_frame().await;
	
	let mut entities: Vec<Box<dyn Entity>> = vec![
		Box::new(Player {
			body: Circle {
				x: 200.0, y: 300.0,
				r: 15.0
			}
		}),
		Box::new(Guy {
			body: Circle {
				x: window::window_width - 30.0,
				y: window::window_height - 30.0,
				r: 30.0
			},
			..Default::default()
		})
	];
	
	loop {
		let tf: f32 = get_frame_time();
		
		let mut all_new_entities = vec![];
		for entity in entities.iter_mut() {
			let entity::Update_Result { mut new_entities } = entity.update(tf);
			all_new_entities.append(&mut new_entities);
		}
		
		let mut to_kill = vec![];
		for (i, entity) in entities.iter().enumerate() {
			if entity.is_dead() {
				to_kill.push(i);
			}
		}
		
		for (n_removed, i) in to_kill.iter().enumerate() {
			entities.remove(i - n_removed);
		}
		
		entities.append(&mut all_new_entities);
		
		println!("{}", entities.len());
		
		clear_background(RED);
		
		for entity in &entities {
			entity.render();
		}
		
		next_frame().await
	}
}