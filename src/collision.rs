pub const number_of_layers: usize = 5;

pub const generic_id: usize = 0;
pub const player_id: usize = 1;
pub const player_bullet_id: usize = 2;
pub const enemy_id: usize = 3;
pub const enemy_bullet_id: usize = 4;

pub const no_checks: &'static [usize] = &[];
pub const with_enemies: &'static [usize] = &[enemy_id];
pub const with_player_bullets: &'static [usize] = &[player_bullet_id];
pub const with_enemy_bullets: &'static [usize] = &[enemy_bullet_id];