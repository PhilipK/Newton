use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {
    forward_thrust_power : f32,
    backwards_thrust_power : f32,
    sideways_thrust_power : f32,
    max_turn_radius : f32, //degrees pr second
}


impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}