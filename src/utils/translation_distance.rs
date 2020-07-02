use amethyst::core::math::Vector3;
use amethyst::core::Transform;

pub fn distance_squared(transform1: &Transform, transform2: &Transform) -> f32 {
    let translate1 = transform1.translation();
    let translate2 = transform2.translation();
    return (translate1.x - translate2.x) * (translate1.x - translate2.x)
        + (translate1.y - translate2.y) * (translate1.y - translate2.y);
}

pub fn distance_squared_vec(vec1: &Vector3<f32>, vec2: &Vector3<f32>) -> f32 {
    return (vec1.x - vec2.x) * (vec1.x - vec2.x) + (vec1.y - vec2.y) * (vec1.y - vec2.y);
}
