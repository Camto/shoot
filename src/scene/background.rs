use crate::entity;
use crate::entity::background::Background;
use crate::scene;


const scroll_speed: f32 = 150.0;

pub fn add(entities: &mut scene::Entities, texs: &entity::Textures) {
	entities[0].append(&mut vec![
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: -texs[0].width(), scroll_speed }),
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: 0.0, scroll_speed }),
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: texs[0].width(), scroll_speed }),
		Box::new(Background { tex_id: 0, tex_width: texs[0].width(), offset: 2.0 * texs[0].width(), scroll_speed })
	])
}