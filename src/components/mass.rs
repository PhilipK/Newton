use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Mass {
    pub mass:f32,
}

impl Component for Mass {
    type Storage = DenseVecStorage<Self>;
}