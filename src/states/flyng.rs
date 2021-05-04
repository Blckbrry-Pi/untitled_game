use super::super::screen_context::ScreenContext;
use super::super::game_objects::level::Level;
use super::State;

use ggez::{GameResult, Context};

pub fn update(
  screen: &mut ScreenContext,
  level: &mut Level,
  millis_since_state_start: u64
) -> Option<State> {
  None
}

pub fn draw(
  ctx: &mut Context,
  screen: &ScreenContext,
  level: &Level,
  _millis_since_state_start: u64
) -> GameResult<()> {
  level.draw_level(ctx, screen)?;

  Ok(())
}