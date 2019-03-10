extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Transform, TransformBundle},
    ecs::{Entity},
    prelude::*,
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, TextureHandle,
        PngFormat, TextureMetadata, Texture, Camera, Projection, SpriteSheet,
        SpriteSheetHandle, SpriteSheetFormat, SpriteRender, Transparent,
        ColorMask, ALPHA, DepthMode
    },
    utils::application_root_dir,
};

#[derive(Default)]
struct PlayState {
    pub knight: Option<Entity>,
}

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let texture_handle = load_texture(world, "background.png");
        let _image = init_image(world, &texture_handle);
        let knight_sheet_handle = load_sprite_sheet(
            world, "knight_spritesheet.png", "knight_spritesheet.ron"
        );
        self.knight = Some(init_knight(world, &knight_sheet_handle));
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
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    // Game Data
    let game_data =
        GameDataBuilder::default()
            .with_bundle(
                TransformBundle::new()
            )?
            .with_bundle(
                RenderBundle::new(pipe, Some(display_config))
                    .with_sprite_sheet_processor()
                    .with_sprite_visibility_sorting(&[]),
            )?;

    // Build and run
    let mut game = Application::new(assets_path, PlayState::default(), game_data)?;
    game.run();
    Ok(())
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

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = load_texture(world, png_path);
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
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

fn init_knight(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(-402.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .build()
}
