use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::math::Vector2;

pub struct Force {
    pub force:Vector2<f32>,
}
impl Component for Force {
    type Storage = DenseVecStorage<Self>;
}