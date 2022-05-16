pub struct Params {
    pub dead_ants: usize,
    pub max_iter: usize,
    pub iter_per_render: usize,
    pub agents: usize,
    pub radius: usize,
    pub threshold: f32,
    pub min_prob: f32,
    pub base_path: String,
    pub colors: Vec<(f32, f32, f32)>,
    pub is_done: bool,
    pub k1: f32,
    pub k2: f32,
    pub alpha: f32,
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
        base_path: String,
        colors: Vec<(f32, f32, f32)>,
        k1: f32,
        k2: f32,
        alpha: f32,
    ) -> Self {
        Self {
            dead_ants,
            agents,
            max_iter,
            iter_per_render,
            radius,
            threshold,
            min_prob,
            base_path,
            colors,
            is_done: false,
            k1,
            k2,
            alpha
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(1000, 10, 10000, 500, 4, 0.45, 0.005, "".to_string(), [].to_vec(), 0.3, 0.9, 30.0)
    }
}
