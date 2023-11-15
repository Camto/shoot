pub mod background;
//pub mod intro;
pub mod level;
pub mod win;
pub mod lose;

use crate::collision;
use crate::entity;
use crate::entity::Entity;


pub type Entities = [Vec<Box<dyn Entity>>; collision::number_of_layers];

#[allow(unused_variables)]
pub trait Scene {
	fn init(&self, texs: &entity::Textures) -> Entities {
		[vec![], vec![], vec![], vec![], vec![]]
	}
}

pub struct Empty_Scene {}

impl Scene for Empty_Scene {}