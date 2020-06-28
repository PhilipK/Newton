use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ScoreBoard {
    pub score: u32,
}

impl Component for ScoreBoard {
    type Storage = DenseVecStorage<Self>;
}
