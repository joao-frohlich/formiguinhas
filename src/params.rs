pub struct Params {
    pub dead_ants: usize,
    pub max_iter: usize,
    pub iter_per_render: usize,
    pub agents: usize,
    pub radius: usize,
    pub threshold: f32,
    pub min_prob: f32,
    pub is_done: bool,
}

impl Params {
    pub fn new(
        dead_ants: usize,
        agents: usize,
        max_iter: usize,
        iter_per_render: usize,
        radius: usize,
        threshold: f32,
        min_prob: f32,
    ) -> Self {
        Self {
            dead_ants,
            agents,
            max_iter,
            iter_per_render,
            radius,
            threshold,
            min_prob,
            is_done: false,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(1000, 10, 10000, 500, 4, 0.45, 0.005)
    }
}
