use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod newton;
mod playercamera;

mod components;
mod entities;
mod systems;

mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear([
                        0.0,
                        0.16 / 4.0,
                        0.21 / 4.0,
                        1.0,
                    ]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerCollisionSystem, "player_collision", &[])
        .with(systems::PlayerControlllerSystem, "player_controller", &[])
        .with(systems::GravitySystem, "gravity", &[])
        .with(
            systems::ForceToAcceletationSystem,
            "force_to_acceleration",
            &["player_controller", "gravity"],
        )
        .with(
            systems::AccelerationToVelocitySystem,
            "acceleration_to_velocity_system",
            &["force_to_acceleration"],
        )
        .with(
            systems::VelocityToTransformSystem,
            "velocity_to_transform_system",
            &["acceleration_to_velocity_system"],
        )
        .with(
            systems::CameraSystem,
            "camera_system",
            &["velocity_to_transform_system"],
        );

    let mut game = Application::new(assets_dir, newton::Newton::default(), game_data)?;
    game.run();

    Ok(())
}
