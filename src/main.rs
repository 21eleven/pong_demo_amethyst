extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};

pub struct Pong;

impl SimpleState for Pong {

}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    use amethyst::utils::application_root_dir;
    let app_root = application_root_dir();
    dbg!(&app_root);
    //let path = app_root.join("resources/display_config.ron");
    let path = app_root+"resources/display_config.ron";
    dbg!(&path);
    let config = DisplayConfig::load(&path);
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );
    
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?;

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}