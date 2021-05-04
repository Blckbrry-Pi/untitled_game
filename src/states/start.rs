use super::super::screen_context::ScreenContext;
use super::super::game_objects::level::Level;
use super::State;

use ggez::{GameResult, Context};
use ggez::graphics::{self, TextFragment, Text};

use std::fs;

pub fn update(
  _screen: &mut ScreenContext,
  _level: &mut Level,
  millis_since_state_start: u64
) -> Option<State> {
  if millis_since_state_start >= 3000 {
    Some(State::Flyng)
  } else {
    None
  }
}

pub fn draw(
  ctx: &mut Context,
  screen: &ScreenContext,
  level: &Level,
  millis_since_state_start: u64
) -> GameResult<()> {
  level.draw_level(ctx, screen)?;

  draw_number(
    ctx,
    screen,
    3 - millis_since_state_start / 1000,
    (millis_since_state_start as f32 / 1000.0) % 1.0
  )?;
  
  Ok(())
}

fn draw_number(
  ctx: &mut Context,
  screen: &ScreenContext,
  number: u64,
  cycle: f32
) -> GameResult {
  let fade_in = (if cycle < 1.0 / 3.0 {cycle} else {1.0 / 3.0}) * 3.0;

  let size = (2.0 - fade_in) * 100.0;

  let colors: Vec<graphics::Color> = vec![
    [1.0, 0.0, 0.0, fade_in].into(),
    [1.0, 1.0, 0.0, fade_in].into(),
    [0.0, 1.0, 0.0, fade_in].into(),
  ];

  let mut text_frag =
    TextFragment::new(number.to_string())
    .color(colors[3 - number as usize])
    .scale(graphics::PxScale::from(size));

  let mut font_path = std::env::current_exe().unwrap();
  font_path.pop();
  font_path.push("../Resources/resources/anton.ttf");

  font_path = font_path.canonicalize().unwrap();

  let font = graphics::Font::new_glyph_font_bytes(ctx, &fs::read(font_path).unwrap())?;

  text_frag = text_frag.font(font);
  
  let text = &Text::new(text_frag);
  
  let text_position = mint::Point2 {x: (screen.size.0 - text.width(ctx)) / 2.0, y: (screen.size.1 - text.height(ctx)) / 2.0};

  graphics::draw(ctx, text, graphics::DrawParam::default().dest(text_position))?;

  Ok(())
}