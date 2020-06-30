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

impl SimpleAnimation {
    pub fn new(number_of_sprites: usize, fps: usize) -> Self {
        SimpleAnimation {
            current_index: 0,
            min_index: 0,
            max_index: number_of_sprites - 1,
            time_pr_index: 1.0 / (fps as f32),
            time_left: 1.0 / (fps as f32),
        }
    }
}
