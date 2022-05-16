#[derive(Default, Clone, Copy)]
pub struct DataItem {
    pub weight: f32,
    pub size: f32,
    pub label: usize,
}

impl DataItem {
    pub fn new(item: (f32, f32, usize)) -> Self {
        let (weight, size, label) = item;
        Self{weight, size, label}
    }
}