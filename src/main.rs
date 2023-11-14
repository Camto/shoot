#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]


pub mod window;
pub mod collision;
pub mod float_utils;
pub mod entity;

use macroquad::prelude::*;
use crate::entity::Entity;
use crate::entity::circle::Circle;
use crate::entity::player::Player;
use crate::entity::guy;
use crate::entity::guy::Guy;
use crate::entity::pew::Pew;


type Entities = [Vec<Box<dyn Entity>>; collision::number_of_layers];

const space_size: f32 = 1024.0;

#[macroquad::main("Shoot")]
async fn main() {
	let space: Texture2D = load_texture("images/space.png").await.unwrap();
	
	request_new_screen_size(window::width, window::height);
	next_frame().await;
	
	let mut entities: Entities = [
		vec![
			Box::new(Guy::new(guy::Guy_Options {
				body: Circle {
					x: window::width - 30.0,
					y: window::height - 30.0,
					r: 30.0
				},
				path: vec![
					(800.0, 100.0),
					(700.0, 200.0),
					(800.0, 300.0),
					(700.0, 400.0)
				],
				..Default::default()
			}))
		],
		vec![
			Box::new(Player {
				body: Circle {
					x: 200.0, y: 300.0,
					r: 15.0
				},
				was_killed: false
			})
		],
		vec![],
		vec![]
	];
	
	loop {
		set_camera(&Camera3D {
			position: vec3(window::width/2.0, window::height/2.0, -500.0),
			target: vec3(window::width/2.0, window::height/2.0, 0.0),
			up: vec3(0.0, -1.0, 0.0),
			..Default::default()
		});
		
		let tf: f32 = get_frame_time();
		
		let mut all_new_entities: Entities = [vec![], vec![], vec![], vec![]];
		for layer in entities.iter_mut() {
			for entity in layer.iter_mut() {
				let entity::Update_Result { new_entities } = entity.update(tf);
				for new_entity in new_entities {
					all_new_entities[new_entity.get_collision_id()].push(new_entity);
				}
			}
		}
		
		let mut num_of_coll_calcs = 0;
		let mut collisions: [Vec<(usize, usize)>; collision::number_of_layers] = [vec![], vec![], vec![], vec![]];
		for (layer_id, layer) in entities.iter().enumerate() {
			for (entity_idx, entity) in layer.iter().enumerate() {
				for &check_layer in entity.checks_collision_with() {
					for (other_idx, other) in entities[check_layer].iter().enumerate() {
						num_of_coll_calcs += 1;
						if entity.collides_with(&**other) {
							collisions[layer_id].push((entity_idx, check_layer));
							collisions[check_layer].push((other_idx, layer_id));
						}
					}
				}
			}
		}
		//println!("# of collision checks: {num_of_coll_calcs}");
		
		for (i, layer) in entities.iter_mut().enumerate() {
			for &(j, collided_id) in collisions[i].iter() {
				layer[j].collided_with(collided_id);
			}
		}
		
		for layer in entities.iter_mut() {
			let mut to_kill = vec![];
			for (i, entity) in layer.iter().enumerate() {
				if entity.is_dead() {
					to_kill.push(i);
				}
			}
			
			for (n_removed, i) in to_kill.iter().enumerate() {
				layer.remove(i - n_removed);
			}
		}
		
		//println!("Entities per layer: {} {} {} {}", entities[0].len(), entities[1].len(), entities[2].len(), entities[3].len());
		
		clear_background(RED);
		
		draw_cube(vec3(window::width * 0.5, window::height * 0.5, 50.0), vec3(space_size, space_size, 1.0), Some(&space), WHITE);
		
		for layer in &entities {
			for entity in layer {
				entity.render();
			}
		}
		
		for (i, layer) in entities.iter_mut().enumerate() {
			layer.append(&mut all_new_entities[i]);
		}
		
		next_frame().await
	}
}