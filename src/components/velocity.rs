use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub velocity: Vector2<f32>,
}
impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            velocity: Vector2::new(x, y),
        }
    }

    pub fn accelerate(self: &mut Self, x: f32, y: f32) {
        let old = self.velocity;
        self.velocity = Vector2::new(x + old.x, y + old.y);
    }
}
