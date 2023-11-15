use crate::window;
use crate::entity;
use crate::entity::circle::Circle;
use crate::entity::player;
use crate::entity::player::Player;
use crate::entity::guy;
use crate::entity::guy::Guy;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


const scroll_speed: f32 = 150.0;

pub struct Level {}

impl Scene for Level {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		let mut entities: scene::Entities = [
			vec![],
			vec![
				Box::new(Player::new(player::Player_Options {
					body: Circle {
						x: 200.0, y: 300.0,
						r: 30.0
					}
				}))
			],
			vec![],
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
			vec![]
		];
		background::add(&mut entities, scroll_speed, &texs);
		entities
	}
}