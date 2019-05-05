extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};

pub struct Pong;

impl SimpleState for Pong {

}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    Ok(())
}