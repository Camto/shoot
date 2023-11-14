use macroquad::prelude::*;


trait Object {
	fn update(&mut self, tf: f32);
	fn render(&self);
}

trait Enemy: Object {
	fn shoot(&mut self, tf: f32) -> Vec<Box<dyn Object>>;
}


struct Circle {
	x: f32, y: f32,
	r: f32
}

impl Object for Circle {
	fn update(&mut self, tf: f32) {}
	
	fn render(&self) {
		draw_circle(self.x, self.y, self.r, YELLOW);
	}
}


const protag_speed: f32 = 300.0;

struct Protag {
	body: Circle
}

impl Object for Protag {
	fn update(&mut self, tf: f32) {
		let speed: f32 =
			if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
				protag_speed * 0.5
			} else {
				protag_speed
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

impl Object for Guy {
	fn update(&mut self, tf: f32) {
		self.body.x -= 5.0 * tf;
	}
	
	fn render(&self) {
		self.body.render();
	}
}

impl Enemy for Guy {
	fn shoot(&mut self, tf: f32) -> Vec<Box<dyn Object>> {
		self.shoot_timer += tf;
		
		if self.shoot_timer >= self.shoot_cycle {
			self.shoot_timer = 0.0;
			
			vec![
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
		} else {
			vec![]
		}
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

impl Object for Pew {
	fn update(&mut self, tf: f32) {
		self.body.x += self.xv * tf;
		self.body.y += self.yv * tf;
	}
	
	fn render(&self) {
		self.body.render();
	}
}


#[macroquad::main("BasicShapes")]
async fn main() {
	request_new_screen_size(1000.0, 600.0);
	next_frame().await;
	
	let mut protag: Protag = Protag {
		body: Circle {
			x: 200.0, y: 300.0,
			r: 15.0
		}
	};
	
	let mut guiz: Vec<Box<dyn Enemy>> = vec![
		Box::new(Guy {
			body: Circle {
				x: 1000.0 - 30.0,
				y: 600.0 - 30.0,
				r: 30.0
			},
			..Default::default()
		})
	];
	
	let mut boulets: Vec<Box<dyn Object>> = vec![];
	
	loop {
		clear_background(RED);
		
		let cam_x_off = lerp(protag.body.x, 0.0, 1000.0, -50.0, 50.0);
		let cam_y_off = lerp(protag.body.y, 0.0, 600.0, -50.0, 50.0);
		println!("{cam_x_off} {cam_y_off}");
		set_camera(&Camera3D {
			position: vec3(500.0 + cam_x_off, 300.0 + cam_y_off, -600.0),
			target: vec3(500.0, 300.0, 0.0),
			up: vec3(0.0, -1.0, 0.0),
			..Default::default()
		});
		
		draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
		draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
		
		let tf: f32 = get_frame_time();
		
		protag.update(tf);
		protag.render();
		
		for gui in guiz.iter_mut() {
			gui.update(tf);
			boulets.append(&mut gui.shoot(tf));
			gui.render();
		}
		
		for bul in boulets.iter_mut() {
			bul.update(tf);
			bul.render();
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