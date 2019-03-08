extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Transform, TransformBundle},
    ecs::{Entity},
    prelude::*,
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, TextureHandle,
        PngFormat, TextureMetadata, Texture, Camera, Projection
    },
    utils::application_root_dir,
};

struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let texture_handle = load_texture(world, "background.png");
        let _image = init_image(world, &texture_handle);
        init_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // Setup paths
    let app_root = application_root_dir();
    let assets_path = format!("{}/assets", app_root);

    // Rendering
    let display_config = DisplayConfig::load(format!("{}/resources/display_config.ron", app_root));
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );
    let render_bundle = RenderBundle::new(pipe, Some(display_config));

    // Game Data
    let game_data =
        GameDataBuilder::default()
            .with_bundle(TransformBundle::new())?
            .with_bundle(render_bundle)?;

    // Build and run
    let mut game = Application::new(assets_path, PlayState, game_data)?;
    game.run();
    Ok(())
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -960.0, 960.0, -540.0, 540.0
        )))
        .with(transform)
        .build();
}

fn load_texture(world: &mut World, png_path: &str) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        png_path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    )
}

fn init_image(world: &mut World, texture: &TextureHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);

    world
        .create_entity()
        .with(transform)
        .with(texture.clone())
        .build()
}
