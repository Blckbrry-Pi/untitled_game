pub mod menus;

pub mod start;
pub mod flyng;

pub mod dying;
pub mod rstrt;

pub mod winng;
pub mod conti;


use super::screen_context::ScreenContext;
use super::game_objects::level::Level;
use ggez::{GameResult, Context};

pub enum State {
  Menus,

  Start,
  Flyng,

  Dying,
  Rstrt,

  Winng,
  Conti,
}

impl State {
  pub fn update(
    &self,
    screen: &mut ScreenContext,
    level: &mut Level,
    millis_since_state_start: u64,
  ) -> Option<State> {
    match self {
      State::Menus => menus::update(screen, level, millis_since_state_start),

      State::Start => start::update(screen, level, millis_since_state_start),
      State::Flyng => flyng::update(screen, level, millis_since_state_start),

      State::Dying => dying::update(screen, level, millis_since_state_start),
      State::Rstrt => rstrt::update(screen, level, millis_since_state_start),

      State::Winng => winng::update(screen, level, millis_since_state_start),
      State::Conti => conti::update(screen, level, millis_since_state_start),
    }
  } 
  
  pub fn draw(
    &self,
    ctx: &mut Context,
    screen: &ScreenContext,
    level: &Level,
    millis_since_state_start: u64,
  ) -> GameResult<()> {
    match self {
      State::Menus => menus::draw(ctx, screen, level, millis_since_state_start),

      State::Start => start::draw(ctx, screen, level, millis_since_state_start),
      State::Flyng => flyng::draw(ctx, screen, level, millis_since_state_start),

      State::Dying => dying::draw(ctx, screen, level, millis_since_state_start),
      State::Rstrt => rstrt::draw(ctx, screen, level, millis_since_state_start),

      State::Winng => winng::draw(ctx, screen, level, millis_since_state_start),
      State::Conti => conti::draw(ctx, screen, level, millis_since_state_start),
    }
  } 
}