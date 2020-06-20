use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::math::Vector2;

pub struct Force {
    pub force:Vector2<f32>,
}
impl Component for Force {
    type Storage = DenseVecStorage<Self>;
}

impl Force{
    pub fn new(x: f32, y: f32) -> Self {
        Force {
            force: Vector2::new(x, y),
        }
    }    

    pub fn add_force(&mut self, x: f32, y: f32) {
        self.force.x += x;
        self.force.y += y;
    }
}