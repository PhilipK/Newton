use amethyst::{assets::Handle, ecs::prelude::World, renderer::SpriteSheet};

#[derive(Clone)]
pub struct SpriteResource {
    pub arrow_sprite_sheet: Handle<SpriteSheet>,
}

pub fn initialise_sprite_resource(
    world: &mut World,
    arrow_sprite_sheet: Handle<SpriteSheet>,
) -> SpriteResource {
    let sprite_resource = SpriteResource {
        arrow_sprite_sheet: arrow_sprite_sheet,
    };

    world.insert(sprite_resource.clone());
    sprite_resource
}
