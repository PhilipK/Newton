use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ScoreArrow {    
}

impl Component for ScoreArrow {
    type Storage = DenseVecStorage<Self>;
}
