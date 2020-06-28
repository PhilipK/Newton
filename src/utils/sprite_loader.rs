use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::{World, WorldExt},
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub fn load_sprite_sheet(world: &World, name: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
