use bevy::prelude::Component;
use crate::data_item::DataItem;

#[derive(Clone, Copy, Component)]
pub struct HeterogeneousCell {
    pub item: Option<DataItem>,
    pub has_alive: bool,
}

impl HeterogeneousCell {
    pub fn new(item: Option<DataItem>) -> Self {
        Self{item, has_alive: false}
    }
}

impl Default for HeterogeneousCell {
    fn default() -> Self {
        Self::new(None)
    }
}