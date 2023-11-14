use macroquad::prelude::*;


trait Entity {
	fn update(&mut self, tf: f32) -> Update_Result {
		Default::default()
	}
	
	fn render(&self) {}
}

struct Update_Result {
	kill: bool,
	new_entities: Vec<Box<dyn Entity>>
}

impl Default for Update_Result {
	fn default() -> Self {
		Update_Result {
			kill: false,
			new_entities: vec![]
		}
	}
}


struct Circle {
	x: f32, y: f32,
	r: f32
}

impl Entity for Circle {
	fn render(&self) {
		draw_circle(self.x, self.y, self.r, YELLOW);
	}
}

impl Circle {
	fn off_screen(&self) -> bool {
		self.x + self.r < -500.0 ||
			self.y + self.r < -500.0 ||
			self.x - self.r > 1500.0 ||
			self.y - self.r > 1100.0
	}
}


const player_speed: f32 = 300.0;

struct Player {
	body: Circle
}

impl Entity for Player {
	fn update(&mut self, tf: f32) -> Update_Result {
		let speed: f32 =
			if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
				player_speed * 0.5
			} else {
				player_speed
			};
		
		if is_key_down(KeyCode::Up) {
			self.body.y -= speed * tf;
		}
		if is_key_down(KeyCode::Down) {
			self.body.y += speed * tf;
		}
		if is_key_down(KeyCode::Left) {
			self.body.x -= speed * tf;
		}
		if is_key_down(KeyCode::Right) {
			self.body.x += speed * tf;
		}
		
		let cam_x_off = lerp(self.body.x, 0.0, 1000.0, -50.0, 50.0);
		let cam_y_off = lerp(self.body.y, 0.0, 600.0, -50.0, 50.0);
		set_camera(&Camera3D {
			position: vec3(500.0 + cam_x_off, 300.0 + cam_y_off, -600.0),
			target: vec3(500.0, 300.0, 0.0),
			up: vec3(0.0, -1.0, 0.0),
			..Default::default()
		});
		
		Default::default()
	}
	
	fn render(&self) {
		self.body.render()
	}
}


struct Guy {
	body: Circle,
	
	shoot_cycle: f32,
	shoot_timer: f32
}

impl Default for Guy {
	fn default() -> Self {
		Guy {
			body: Circle { x: 100.0, y: 50.0, r: 30.0 },
			shoot_cycle: 2.5,
			shoot_timer: 2.0
		}
	}
}

impl Entity for Guy {
	fn update(&mut self, tf: f32) -> Update_Result {
		self.body.x -= 5.0 * tf;
		
		self.shoot_timer += tf;
		if self.shoot_timer >= self.shoot_cycle {
			self.shoot_timer = 0.0;
			
			Update_Result {
				kill: false,
				new_entities: vec![
					Box::new(Pew {
						body: Circle { x: self.body.x, y: self.body.y - 20.0, r: 10.0 },
						yv: -30.0,
						..Default::default()
					}),
					Box::new(Pew {
						body: Circle { x: self.body.x, y: self.body.y, r: 10.0 },
						..Default::default()
					}),
					Box::new(Pew {
						body: Circle { x: self.body.x, y: self.body.y + 20.0, r: 10.0 },
						yv: 30.0,
						..Default::default()
					})
				]
			}
		} else {
			Default::default()
		}
	}
	
	fn render(&self) {
		self.body.render();
	}
}


struct Pew {
	body: Circle,
	xv: f32, yv: f32
}

impl Default for Pew {
	fn default() -> Self {
		Pew {
			body: Circle { x: 100.0, y: 50.0, r: 10.0 },
			xv: -200.0, yv: 0.0
		}
	}
}

impl Entity for Pew {
	fn update(&mut self, tf: f32) -> Update_Result {
		self.body.x += self.xv * tf;
		self.body.y += self.yv * tf;
		
		Update_Result {
			kill: self.body.off_screen(),
			..Default::default()
		}
	}
	
	fn render(&self) {
		self.body.render();
	}
}


#[macroquad::main("Shoot")]
async fn main() {
	request_new_screen_size(1000.0, 600.0);
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
				x: 1000.0 - 30.0,
				y: 600.0 - 30.0,
				r: 30.0
			},
			..Default::default()
		})
	];
	
	loop {
		let tf: f32 = get_frame_time();
		
		let mut all_new_entities = vec![];
		let mut to_kill = vec![];
		for (i, entity) in entities.iter_mut().enumerate() {
			let Update_Result { kill, mut new_entities } = entity.update(tf);
			
			if kill {
				to_kill.push(i);
			}
			all_new_entities.append(&mut new_entities);
		}
		
		for (n_removed, i) in to_kill.iter().enumerate() {
			entities.remove(i - n_removed);
		}
		
		entities.append(&mut all_new_entities);
		
		clear_background(RED);
		
		draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
		draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
		
		for entity in &entities {
			entity.render();
		}
		
		draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
		
		next_frame().await
	}
}

fn lerp(n: f32, a1: f32, b1: f32, a2: f32, b2: f32) -> f32 {
	let normalized = (n - a1) / (b1 - a1);
	/* let evened = normalized * 2.0 - 1.0;
	let extremified = evened.abs().sqrt().copysign(evened);
	let renormalized = (extremified + 1.0) * 0.5; */
	normalized * (b2 - a2) + a2
}