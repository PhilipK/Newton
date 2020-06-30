use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct SimpleAnimation {
    pub current_index: usize,
    pub min_index: usize,
    pub max_index: usize,
    pub time_pr_index: f32,
    pub time_left: f32,
}

impl Component for SimpleAnimation {
    type Storage = DenseVecStorage<Self>;
}
