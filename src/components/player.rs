use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {
    pub forward_thrust_power: f32,
    pub backwards_thrust_power: f32,
    pub sideways_thrust_power: f32,
    pub turn_pr_second: f32,
    pub is_dead: bool,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
