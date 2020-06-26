use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ZoomCamera {
    pub zoom_level: f32,
}

impl Component for ZoomCamera {
    type Storage = DenseVecStorage<Self>;
}
