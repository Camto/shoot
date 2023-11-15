use crate::entity;
use crate::entity::Entity;
use crate::entity::background::Background;
use crate::scene;


pub fn add(entities: &mut scene::Entities, scroll_speed: f32, texs: &entity::Textures) {
	let mut prepended_entities: Vec<Box<dyn Entity>> = vec![
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: -texs[0].width(), scroll_speed }),
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: 0.0, scroll_speed }),
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: texs[0].width(), scroll_speed }),
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: 2.0 * texs[0].width(), scroll_speed })
	];
	prepended_entities.append(&mut entities[0]);
	entities[0] = prepended_entities
}