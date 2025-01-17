use crate::components::Destroyable;
use crate::components::Player;
use crate::components::ScoreArrow;
use crate::components::SimpleAnimation;
use crate::components::ZoomCamera;
use crate::entities::high_score_text;
use crate::entities::score_board::ScoreBoard;
use crate::resources::sprite::SpriteResource;

use amethyst_core::ArcThreadPool;

use crate::entities::player;
use crate::entities::score_area;
use crate::entities::score_board;
use crate::entities::title_text;

use amethyst::core::transform::Transform;
use amethyst::ecs::world::LazyBuilder;

use crate::playercamera;
use amethyst::ecs::Entities;
use amethyst::ecs::{Entity, Join, Read, ReadExpect, ReadStorage, Write};

use crate::{systems::*, utils::world_generator::generate_world};
use amethyst::ecs::prelude::{Dispatcher, DispatcherBuilder};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    prelude::{GameData, SimpleState, SimpleTrans, StateData},
    renderer::{SpriteRender, SpriteSheet},
    ui::UiText,
};
use amethyst_rendy::{palette::Srgba, resources::Tint};

#[derive(Default)]
pub struct Newton<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    came_from_pause: bool,
}

impl Newton<'_, '_> {
    fn load_planets(&mut self, world: &mut World) {
        generate_world(world);
    }
}

#[derive(Default)]
struct PauseState {
    pub was_release: bool,
}

impl<'a, 'b> SimpleState for PauseState {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let input: Read<InputHandler<StringBindings>> = data.world.system_data();
        if let Some(is_down) = input.action_is_down("pause") {
            match (is_down, self.was_release) {
                (true, true) => return Trans::Pop,
                (false, true) => (),
                (false, false) => self.was_release = true,
                (true, false) => (),
            };
        }
        Trans::None
    }
}

impl<'a, 'b> SimpleState for Newton<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Starting Newton");
        let world = data.world;
        let (player_sprite, score_area) = {
            let sprites: ReadExpect<SpriteResource> = world.system_data();
            let player_sprite = sprites
                .player_sprite_sheet_handle
                .clone()
                .expect("Sprite sheet was not loaded before game was started");
            let score_area = sprites.score_area_sprite_sheet_handle.clone().unwrap();
            (player_sprite, score_area)
        };

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
        dispatcher_builder.add(WrapAroundSystem, "wrap_around", &[]);

        // Build and setup the `Dispatcher`.
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
        score_board::initialise_scoreboard(world);
        player::initialize_player(world, player_sprite);
        self.load_planets(world);
        score_area::initialize_score_area(world, score_area);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let entities: Entities = data.world.system_data();
        let destroyable: ReadStorage<Destroyable> = data.world.system_data();
        for (_, entity) in (&destroyable, &entities).join() {
            let _unused = entities.delete(entity);
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;
        let input: Read<InputHandler<StringBindings>> = data.world.system_data();

        if let Some(is_down) = input.action_is_down("pause") {
            if !self.came_from_pause && is_down {
                self.came_from_pause = true;
                return Trans::Push(Box::new(PauseState::default()));
            }
            if !is_down {
                self.came_from_pause = false;
            }
        }

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
        title_text::itnitialize_title_text(world);
        initialize_star_field(world);
        world.register::<ZoomCamera>();
        world.register::<Destroyable>();
        playercamera::initialize_camera(world);
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
        let ui_text: ReadStorage<UiText> = data.world.system_data();
        let entities: Entities = data.world.system_data();
        for (_ui, entity) in (&ui_text, &entities).join() {
            let _unused = entities.delete(entity);
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
        let ui_text: ReadStorage<UiText> = data.world.system_data();
        let entities: Entities = data.world.system_data();
        for (_ui, entity) in (&ui_text, &entities).join() {
            let _unused = entities.delete(entity);
        }
    }
}

fn initialize_star_field(world: &mut World) {
    let star_field_sheet = {
        let sprites: ReadExpect<SpriteResource> = world.system_data();
        sprites.star_field_sheet_handle.clone().unwrap()
    };
    let width = NUMBER_OF_TILES as u32 + 2;
    let height = width;
    let sprite_size = SPRITE_SIZE;
    let offset = sprite_size / 2.0 - sprite_size;
    for i in 0..(width * height) {
        let mut transform = Transform::default();
        let (sprite_x, sprite_y) = ((i % width), ((i / width) as u32));
        transform.set_translation_xyz(
            (sprite_x as f32) * sprite_size + offset,
            (sprite_y as f32) * sprite_size + offset,
            -0.1,
        );

        //Sprite renderer
        let sprite_render = SpriteRender {
            sprite_sheet: star_field_sheet.clone(),
            sprite_number: 0,
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
    sprite_sheet_handle: Handle<SpriteSheet>,
) -> Entity {
    //create score arrow
    let mut arrow_transform = Transform::default();
    let cur_x = current_pos.translation().x;
    let cur_y = current_pos.translation().y;
    arrow_transform.set_translation_xyz(cur_x, cur_y, 0.0);
    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // default ship is 0
    };

    let animation = SimpleAnimation::new(5, 5);
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 0.0));
    return builder
        .with(arrow_transform)
        .with(ScoreArrow {})
        .with(tint)
        .with(sprite_render)
        .with(animation)
        .build();
}
