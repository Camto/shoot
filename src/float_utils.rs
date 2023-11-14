pub fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
	let x_dist = x2 - x1;
	let y_dist = y2 - y1;
	(x_dist * x_dist + y_dist * y_dist).sqrt()
}

pub fn lerp(n: f32, a1: f32, b1: f32, a2: f32, b2: f32) -> f32 {
	let normalized = (n - a1) / (b1 - a1);
	/* let evened = normalized * 2.0 - 1.0;
	let extremified = evened.abs().sqrt().copysign(evened);
	let renormalized = (extremified + 1.0) * 0.5; */
	normalized * (b2 - a2) + a2
}

pub fn quad_ease(n: f32) -> f32 {
	if n <= 0.5 {
		2.0 * n * n
	} else {
		let n = n - 0.5;
		2.0 * n * (1.0 - n) + 0.5
	}
}