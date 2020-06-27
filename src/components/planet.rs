use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Planet {
    pub radius: f32,
    pub radius_squared: f32,
}
impl Component for Planet {
    type Storage = DenseVecStorage<Self>;
}

impl Planet {
    pub fn new(radius: f32) -> Self {
        Planet {
            radius: radius,
            radius_squared: radius * radius,
        }
    }
}
