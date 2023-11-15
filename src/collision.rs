pub const number_of_layers: usize = 4;

pub const generic_id: usize = 0;
pub const player_id: usize = 1;
pub const bullet_id: usize = 2;
pub const enemy_id: usize = 3;

pub const no_checks: &'static [usize] = &[];
pub const with_bullets: &'static [usize] = &[bullet_id];
pub const with_enemies: &'static [usize] = &[enemy_id];