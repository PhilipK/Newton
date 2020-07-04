use crate::components::Player;
use crate::components::ScoreArrow;
use crate::components::SimpleAnimation;
use crate::entities::high_score_text;
use crate::entities::score_board::ScoreBoard;

use amethyst_core::ArcThreadPool;

use crate::components::Velocity;
use crate::entities::player;
use crate::entities::score_area;
use crate::entities::score_board;
use crate::entities::star;
use crate::entities::title_text;
use crate::playercamera;
use crate::resources::initialise_sprite_resource;
use crate::resources::initialize_audio;

use amethyst::core::math::Vector2;
use amethyst::core::transform::Transform;
use amethyst::ecs::world::LazyBuilder;
use std::f64::consts::PI;

use crate::utils::load_sprite_sheet;
use amethyst::ecs::Entities;
use amethyst::ecs::{Entity, Join, Read, ReadStorage, Write};

use crate::systems::*;
use amethyst::ecs::prelude::{Dispatcher, DispatcherBuilder};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    prelude::{GameData, SimpleState, SimpleTrans, StateData},
    renderer::{SpriteRender, SpriteSheet},
};

#[derive(Default)]
pub struct Newton<'a, 'b> {
    player_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    star_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    earth_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    meteor_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    score_area_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    score_arrow_sheet_handle: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    star_field_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl Newton<'_, '_> {
    fn load_sprite_sheets(&mut self, world: &mut World) {
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
        self.score_arrow_sheet_handle
            .replace(load_sprite_sheet(world, "next_arrow"));
        self.star_field_sheet_handle
            .replace(load_sprite_sheet(world, "star_field_big"));
    }

    fn load_planets(&mut self, world: &mut World) {
        let meteor_number = 20;
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
                4,
                1,
            );
        }

        star::initialize_star(
            world,
            1000000.0,
            64.0,
            0.0,
            0.0,
            -100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
            4,
            5,
        );

        star::initialize_star(
            world,
            1000000.0,
            64.0,
            -200.0,
            0.0,
            -100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
            4,
            5,
        );

        star::initialize_star(
            world,
            1000000.0,
            62.0,
            900.0,
            900.0,
            100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
            4,
            5,
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
            7,
            5,
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
            7,
            5,
        );
    }
}

impl<'a, 'b> SimpleState for Newton<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Create the `DispatcherBuilder` and register some `System`s that should only run for this `State`.
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(PlayerCollisionSystem, "player_collision", &[]);
        dispatcher_builder.add(PlayerControlllerSystem, "player_controller", &[]);
        dispatcher_builder.add(PlayerAnimationSystem, "player_animation", &[]);
        dispatcher_builder.add(SimpleAnimationSystem, "simple_animation", &[]);
        dispatcher_builder.add(GravitySystem, "gravity", &[]);
        dispatcher_builder.add(ScoreArrowSystem, "score_arrow_system", &[]);
        dispatcher_builder.add(ScoreSystem, "score_system", &[]);
        dispatcher_builder.add(
            ForceToAcceletationSystem,
            "force_to_acceleration",
            &["player_controller", "gravity"],
        );
        dispatcher_builder.add(
            AccelerationToVelocitySystem,
            "acceleration_to_velocity_system",
            &["force_to_acceleration"],
        );
        dispatcher_builder.add(
            VelocityToTransformSystem,
            "velocity_to_transform_system",
            &["acceleration_to_velocity_system"],
        );
        dispatcher_builder.add(
            CameraSystem,
            "camera_system",
            &["velocity_to_transform_system"],
        );

        // Build and setup the `Dispatcher`.
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        score_board::initialise_scoreboard(world);
        self.load_sprite_sheets(world);
        initialise_sprite_resource(world, self.score_arrow_sheet_handle.clone().unwrap());
        player::initialize_player(world, self.player_sprite_sheet_handle.clone().unwrap());
        self.load_planets(world);
        score_area::initialize_score_area(
            world,
            self.score_area_sprite_sheet_handle.clone().unwrap(),
        );
        playercamera::initialize_camera(world);
        initialize_star_field(world, self.star_field_sheet_handle.clone().unwrap());
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let system_data: Entities = data.world.system_data();
        for entity in (&system_data).join() {
            let _unused = system_data.delete(entity);
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }
        let players: ReadStorage<Player> = world.system_data();
        for player in (&players).join() {
            if player.is_dead {
                let score = {
                    let mut system_data: Write<ScoreBoard> = world.system_data();
                    let res = system_data.score;
                    system_data.score = 0;
                    res
                };

                let high_score_state = HighScoreScreen { score };
                return Trans::Replace(Box::new(high_score_state));
            }
        }

        Trans::None
    }
}

#[derive(Default)]
pub struct TitleScreen {}

impl<'a, 'b> SimpleState for TitleScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_audio(world);
        title_text::itnitialize_title_text(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let input: Read<InputHandler<StringBindings>> = data.world.system_data();
        if let Some(value) = input.axis_value("throttle") {
            if value > 0.0 {
                return Trans::Replace(Box::new(Newton::default()));
            }
        }

        Trans::None
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let system_data: Entities = data.world.system_data();
        for entity in (&system_data).join() {
            let _unused = system_data.delete(entity);
        }
    }
}

#[derive(Default)]
pub struct HighScoreScreen {
    pub score: i32,
}

impl<'a, 'b> SimpleState for HighScoreScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        high_score_text::itnitialize_highscore_text(world, self.score);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let input: Read<InputHandler<StringBindings>> = data.world.system_data();
        if let Some(value) = input.axis_value("throttle") {
            if value < 0.0 {
                return Trans::Replace(Box::new(Newton::default()));
            }
        }
        Trans::None
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let system_data: Entities = data.world.system_data();
        for entity in (&system_data).join() {
            let _unused = system_data.delete(entity);
        }
    }
}

fn initialize_star_field(world: &mut World, sheet: Handle<SpriteSheet>) {
    let width = 20;
    let height = 20;
    let sprite_size = 512.0;
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
    let speed = 250.0;
    let velocity = Velocity::new(x / mag * speed, y / mag * speed);
    let vel_vec: Vector2<f32> = velocity.velocity;
    let up = Vector2::new(0.0, 1.0);
    let mut angle = up.angle(&vel_vec);
    println!("a1 {}", up.angle(&vel_vec));
    if vel_vec.x > 0.0 {
        angle = 2.0 * PI as f32 - angle;
    }
    println!("a2 {}", vel_vec.angle(&up));
    arrow_transform.set_rotation_2d(angle);

    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // default ship is 0
    };

    let animation = SimpleAnimation::new(5, 5);

    return builder
        .with(arrow_transform)
        .with(ScoreArrow {})
        .with(sprite_render)
        .with(velocity)
        .with(animation)
        .build();
}
