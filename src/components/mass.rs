use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Mass {
    pub mass:f32,
}

impl Component for Mass {
    type Storage = DenseVecStorage<Self>;
}

impl Mass{
    pub fn new(mass: f32) -> Self {
        Mass {
            mass 
        }
    }
}