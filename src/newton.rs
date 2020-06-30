use crate::components::Player;
use crate::components::ScoreArrow;

use crate::components::Velocity;
use crate::entities::player;
use crate::entities::score_area;
use crate::entities::score_board;
use crate::entities::star;
use crate::playercamera;
use crate::resources::initialise_sprite_resource;
use amethyst::core::math::Matrix;
use amethyst::core::math::angle;
use amethyst::core::math::Vector2;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::world::LazyBuilder;

use crate::utils::load_sprite_sheet;
use amethyst::ecs::Entities;
use amethyst::ecs::{Entity, Join, ReadStorage};

use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    prelude::{GameData, SimpleState, SimpleTrans, StateData},
    renderer::{SpriteRender, SpriteSheet},
};

#[derive(Default)]
pub struct Newton {
    player_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    star_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    earth_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    meteor_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    score_area_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    star_field_sheet_handle: Option<Handle<SpriteSheet>>,
    score_arrow_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl Newton {
    fn new() -> Self {
        Newton {
            player_sprite_sheet_handle: Option::None,
            star_sprite_sheet_handle: Option::None,
            earth_sprite_sheet_handle: Option::None,
            meteor_sprite_sheet_handle: Option::None,
            score_area_sprite_sheet_handle: Option::None,
            star_field_sheet_handle: Option::None,
            score_arrow_sheet_handle: Option::None,
        }
    }
}

impl SimpleState for Newton {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        score_board::initialise_scoreboard(world);

        self.player_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "player_spritesheet"));
        self.star_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "star"));
        self.earth_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "earth"));
        self.meteor_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "meteor"));
        self.score_area_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "score_area"));
        self.star_field_sheet_handle
            .replace(load_sprite_sheet(world, "star_field_big"));
        self.score_arrow_sheet_handle
            .replace(load_sprite_sheet(world, "next_arrow"));

        initialise_sprite_resource(world, self.score_arrow_sheet_handle.clone().unwrap());
        player::initialize_player(world, self.player_sprite_sheet_handle.clone().unwrap());

        let meteor_number = 10;
        for i in 0..meteor_number {
            star::initialize_star(
                world,
                10.0,
                16.0,
                (10000.0 / meteor_number as f32) * (i as f32),
                (10000.0 / meteor_number as f32) * (i as f32),
                0.0,
                0.0,
                self.meteor_sprite_sheet_handle.clone().unwrap(),
            );
        }

        star::initialize_star(
            world,
            1000000.0,
            64.0,
            200.0,
            200.0,
            -100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
        );

        star::initialize_star(
            world,
            1000000.0,
            62.0,
            700.0,
            700.0,
            100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
        );
        star::initialize_star(
            world,
            1000.0,
            32.0,
            800.0,
            800.0,
            100.0,
            -80.0,
            self.earth_sprite_sheet_handle.clone().unwrap(),
        );
        star::initialize_star(
            world,
            1000.0,
            32.0,
            100.0,
            100.0,
            -100.0,
            80.0,
            self.earth_sprite_sheet_handle.clone().unwrap(),
        );
        score_area::initialize_score_area(
            world,
            self.score_area_sprite_sheet_handle.clone().unwrap(),
        );

        initialize_star_field(world, self.star_field_sheet_handle.clone().unwrap());

        playercamera::initialize_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let system_data: Entities = data.world.system_data();
        for entity in (&system_data).join() {
            let _unused = system_data.delete(entity);
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let system_data: ReadStorage<Player> = data.world.system_data();
        for player in (&system_data).join() {
            if player.is_dead {
                return Trans::Replace(Box::new(Newton::new()));
            }
        }

        Trans::None
    }
}
fn initialize_star_field(world: &mut World, sheet: Handle<SpriteSheet>) {
    let width = 20;
    let height = 20;
    let sprite_size = 512.0;
    // let offset = -1.0 * (width as f32) / 2.0 * (height as f32) / 2.0 * sprite_size;
    let offset = (width as f32) * -0.5 * sprite_size;
    for i in 0..(width * height) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (i % width) as f32 * sprite_size + offset,
            ((i / width) as u32) as f32 * sprite_size + offset,
            -0.1,
        );
        //Sprite renderer
        let sprite_render = SpriteRender {
            sprite_sheet: sheet.clone(),
            sprite_number: 0, // default ship is 0
        };
        world
            .create_entity()
            .with(transform)
            .with(sprite_render)
            .build();
    }
}

pub fn spawn_score_arrow(
    builder: LazyBuilder,
    current_pos: &Transform,
    target_pos: &Transform,
    sprite_sheet_handle: Handle<SpriteSheet>,
) -> Entity {
    //create score arrow
    let mut arrow_transform = Transform::default();
    let cur_x = current_pos.translation().x;
    let cur_y = current_pos.translation().y;
    let target_x = target_pos.translation().x;
    let target_y = target_pos.translation().y;
    arrow_transform.set_translation_xyz(cur_x, cur_y, 0.0);

    let (x, y) = (target_x - cur_x, target_y - cur_y);
    let mag = ((x * x) + (y * y)).sqrt();
    let speed = 100.0;
    let velocity = Velocity::new(x / mag * speed, y / mag * speed);
    let vel_vec = velocity.velocity;
    let forward = Vector2::new(0.0, 1.0);
    let angle = angle(&vel_vec, &forward);
    arrow_transform.rotate_2d(angle);

    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 3, // default ship is 0
    };

    return builder
        .with(arrow_transform)
        .with(ScoreArrow {})
        .with(sprite_render)
        .with(velocity)
        .build();
}
