use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Gravity {}

impl Component for Gravity {
    type Storage = DenseVecStorage<Self>;
}
