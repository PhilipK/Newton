use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Destroyable {
}
impl Component for Destroyable {
    type Storage = DenseVecStorage<Self>;
}