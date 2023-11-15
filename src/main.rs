#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]


pub mod window;
pub mod collision;
pub mod float_utils;
pub mod entity;
pub mod scene;

use macroquad::prelude::*;
use crate::scene::Scene;
use crate::scene::start::Start;


#[macroquad::main("Shoot")]
async fn main() {
	let mut texs: entity::Textures = vec![];
	for s in [
		"assets/space.png",
		
		"assets/player.png",
		"assets/player_damage_1.png", "assets/player_damage_2.png", "assets/player_damage_3.png",
		"assets/player_bullet_1.png", "assets/player_bullet_2.png",
		
		"assets/guy.png",
		"assets/guy_bullet_1.png", "assets/guy_bullet_2.png"
	] {
		texs.push(load_texture(s).await.unwrap());
	}
	
	let sounds: entity::Sounds = vec![];
	
	let kenvector_future: Font = load_ttf_font("assets/kenvector_future.ttf").await.unwrap();
	
	request_new_screen_size(window::width, window::height);
	next_frame().await;
	
	let mut entities: scene::Entities = (Start {}).init(&texs);
	
	loop {
		set_camera(&Camera3D {
			position: vec3(window::width/2.0, window::height/2.0, -500.0),
			target: vec3(window::width/2.0, window::height/2.0, 0.0),
			up: vec3(0.0, -1.0, 0.0),
			..Default::default()
		});
		
		let tf: f32 = get_frame_time();
		
		let mut all_new_entities: scene::Entities = (scene::Empty_Scene {}).init(&texs);
		let mut new_scene: Option<Box<dyn Scene>> = None;
		for layer in entities.iter_mut() {
			for entity in layer.iter_mut() {
				let entity::Update_Result { new_entities, change_scene } = entity.update(tf);
				
				for new_entity in new_entities {
					all_new_entities[new_entity.get_collision_id()].push(new_entity);
				}
				
				if change_scene.is_some() {
					new_scene = change_scene
				}
			}
		}
		
		let mut num_of_coll_calcs = 0;
		let mut collisions: [Vec<(usize, usize)>; collision::number_of_layers] = [vec![], vec![], vec![], vec![], vec![]];
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
				if !layer[j].is_dead() {
					layer[j].collided_with(collided_id);
				}
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
		
		for layer in &entities {
			for entity in layer {
				entity.render(&texs, &sounds, &kenvector_future);
			}
		}
		
		for (i, layer) in entities.iter_mut().enumerate() {
			layer.append(&mut all_new_entities[i]);
		}
		
		if let Some(scene) = new_scene {
			entities = scene.init(&texs)
		}
		
		next_frame().await
	}
}