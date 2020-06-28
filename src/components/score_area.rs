use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ScoreArea {
    pub time_left: f32,
}
impl Component for ScoreArea {
    type Storage = DenseVecStorage<Self>;
}

impl ScoreArea {
    pub fn new(target_time: f32) -> Self {
        ScoreArea {
            time_left: target_time,
        }
    }
}
