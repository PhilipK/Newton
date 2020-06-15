use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Acceleration {
    pub acceleration: Vector2<f32>,
}

impl Component for Acceleration {
    type Storage = DenseVecStorage<Self>;
}

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Acceleration {
            acceleration: Vector2::new(x, y),
        }
    }
}
