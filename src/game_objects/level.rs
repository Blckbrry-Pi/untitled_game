use super::super::get_resource_folder;
use super::zipper::Zipper;
use super::attractor::Attractor;

use super::super::screen_context::ScreenContext;

use ggez::{GameResult, Context};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Level {
  pub attractors: Vec<Attractor>,
  pub zippers: Vec<Zipper>,
}

impl Level {
  pub fn draw_level(
    &self,
    ctx: &mut Context,
    screen: &ScreenContext
  ) -> GameResult<()> {
    for zipper in self.zippers.iter() {
      zipper.draw(ctx, screen)?;
    }
    for attractor in self.attractors.iter() {
      attractor.draw(ctx, screen)?;
    }
    Ok(())
  }

  pub fn load_level(
    &mut self,
    level_ind: usize
  ) {
    let mut path = std::path::PathBuf::from(get_resource_folder());
    path.push("resources/levels.levelData");
    
    let file_string = &std::fs::read_to_string(path).unwrap();

    let mut json_string = file_string.split("\n").skip(level_ind);

    let new_level: Self = serde_json::from_str(json_string.next().unwrap()).unwrap();

    self.attractors = new_level.attractors;
    self.zippers = new_level.zippers;
  }

}