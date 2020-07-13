use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::{World, WorldExt},
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Clone, Default)]
pub struct SpriteResource {
    pub score_arrow_sheet_handle: Option<Handle<SpriteSheet>>,
    pub player_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub star_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub earth_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub meteor_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub score_area_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub star_field_sheet_handle: Option<Handle<SpriteSheet>>,
    pub mars_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub saturn_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub uranus_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

pub fn initialise_sprite_resource(world: &mut World, progress_counter: &mut ProgressCounter) {
    let mut sprite_resource = SpriteResource::default();
    sprite_resource.load_sprite_sheets(world, progress_counter);
    world.insert(sprite_resource);
}

impl SpriteResource {
    fn load_sprite_sheets(&mut self, world: &mut World, progress_counter: &mut ProgressCounter) {
        self.player_sprite_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "player_spritesheet",
        ));
        self.star_sprite_sheet_handle
            .replace(load_sprite_sheet(world, progress_counter, "star"));
        self.earth_sprite_sheet_handle
            .replace(load_sprite_sheet(world, progress_counter, "earth"));
        self.meteor_sprite_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "meteor",
        ));
        self.score_area_sprite_sheet_handle
            .replace(load_sprite_sheet(world, progress_counter, "score_area"));
        self.score_arrow_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "next_arrow",
        ));
        self.star_field_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "star_field_big",
        ));
        self.mars_sprite_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "mars",
        ));
        self.saturn_sprite_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "saturn",
        ));
        self.uranus_sprite_sheet_handle.replace(load_sprite_sheet(
            world,
            progress_counter,
            "uranus",
        ));
    }
}

pub fn load_sprite_sheet(
    world: &World,
    progress_counter: &mut ProgressCounter,
    name: &str,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}.png", name),
            ImageFormat::default(),
            progress_counter,
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
