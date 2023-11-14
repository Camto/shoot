pub const number_of_layers: usize = 4;

pub const generic_coll_id: usize = 0;
pub const player_coll_id: usize = 1;
pub const bullet_coll_id: usize = 2;
pub const enemy_coll_id: usize = 3;

pub const no_coll_checks: &'static [usize] = &[];
pub const collide_with_enemies: &'static [usize] = &[enemy_coll_id];