use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod player;
mod newton;
mod playercamera;

mod systems;
mod components;



fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.168627451, 0.2117647059, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(systems::VelocityToTransformSystem, "velocity_to_transform_system", &[])
        .with(systems::AccelerationToVelocitySystem, "acceleration_to_velocity_system", &["velocity_to_transform_system"]);

    let mut game = Application::new(assets_dir, newton::Newton::default(), game_data)?;
    game.run();

    Ok(())
}
