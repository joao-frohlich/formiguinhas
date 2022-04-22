use bevy::prelude::Component;

#[derive(Default, Clone, Copy, Component)]
pub struct Cell {
    pub has_alive: bool,
    pub has_dead: bool,
}
