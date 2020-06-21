use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {
    pub forward_thrust_power: f32,
    // backwards_thrust_power : f32,
    // sideways_thrust_power : f32,
     pub turn_pr_second : f32, 
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
