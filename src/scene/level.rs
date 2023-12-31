use crate::window;
use crate::entity;
use crate::entity::circle::Circle;
use crate::entity::player;
use crate::entity::player::Player;
use crate::entity::enemy;
use crate::entity::enemy::Enemy;
use crate::entity::win_checker::Win_Checker;
use crate::scene;
use crate::scene::Scene;
use crate::scene::background;


const scroll_speed: f32 = 150.0;

pub struct Level {}

impl Scene for Level {
	fn init(&self, texs: &entity::Textures) -> scene::Entities {
		let mut entities: scene::Entities = [
			vec![
				Box::new(Win_Checker::new())
			],
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
				Box::new(Enemy::new(enemy::Enemy_Options {
					body: Circle {
						x: window::width + 100.0,
						y: -100.0,
						r: 30.0
					},
					path: vec![
						(800.0, 100.0),
						(700.0, 200.0),
						(800.0, 300.0),
						(700.0, 400.0)
					],
					..Default::default()
				})),
				Box::new(Enemy::new(enemy::Enemy_Options {
					body: Circle {
						x: window::width + 100.0,
						y: window::mid_height,
						r: 30.0
					},
					path: vec![
						(650.0, 200.0),
						(550.0, 300.0),
						(650.0, 400.0),
						(550.0, 500.0)
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