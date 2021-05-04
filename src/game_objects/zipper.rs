use ggez::{graphics};
use ggez::{Context, GameResult};

use super::point::Point;
use super::super::screen_context::ScreenContext;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectionalLine {
  start_point: Point,
  end_point: Point
}

#[allow(clippy::float_cmp)]
impl DirectionalLine {
  pub fn new(
    p_start: &Point,
    p_end: &Point,
  ) -> DirectionalLine {
    DirectionalLine {
      start_point: *p_start,
      end_point: *p_end,
    }
  }

  pub fn length(&self) -> f32 {
    let x_diff = self.start_point.x - self.end_point.x;
    let y_diff = self.start_point.y - self.end_point.y;
    (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt()
  }

  pub fn is_vertical(&self) -> bool {
    self.start_point.x == self.end_point.x
  }

  pub fn slope(&self) -> f32 {
    if self.is_vertical() {
      f32::INFINITY
    } else {
      (self.end_point.y - self.start_point.y) / (self.end_point.x - self.start_point.x)
    }
  }

  pub fn slope_int_format(&self) -> (f32, f32) {
    let slope = self.slope();
    let int = if self.is_vertical() {
      f32::NAN
    } else {
      self.start_point.y - self.start_point.x * slope
    };

    (slope, int)
  }

  pub fn point_is_on_line(
    &self,
    point: &Point,
  ) -> bool {
    let x_bounds = if self.start_point.x < self.end_point.x {
      (self.start_point.x, self.end_point.x)
    } else {
      (self.end_point.x, self.start_point.x)
    };

    let y_bounds = if self.start_point.y < self.end_point.y {
      (self.start_point.y, self.end_point.y)
    } else {
      (self.end_point.y, self.start_point.y)
    };

    let x_in_bounds = x_bounds.0 <= point.x && point.x <= x_bounds.1;
    let y_in_bounds = y_bounds.0 <= point.y && point.y <= y_bounds.1;
    

    let s_i_f = if self.is_vertical() {
      self.slope_int_format()
    } else {
      (0.0, point.y)
    };

    let on_line = point.y == point.x * s_i_f.0 + s_i_f.1;

    x_in_bounds && y_in_bounds && on_line
  }

  pub fn calc_line_intersect(
    &self,
    other_line: &DirectionalLine,
  ) -> Option<Point> {
    if self.length() == 0.0 || other_line.length() == 0.0 {
      return None;
    }

    if self.is_vertical() && other_line.is_vertical() {
      None
    } else if self.is_vertical() || other_line.is_vertical() {
      if self.is_vertical() {
        let s_i_f = other_line.slope_int_format();
        let y_coord = s_i_f.0 * self.start_point.x + s_i_f.1;
        Some(Point {x: self.start_point.x, y: y_coord})
      } else  {
        let s_i_f = self.slope_int_format();
        let y_coord = s_i_f.0 * other_line.start_point.x + s_i_f.1;
        Some(Point {x: other_line.start_point.x, y: y_coord})
      }
    } else {
      let self_s_i_f = self.slope_int_format();
      let other_s_i_f = other_line.slope_int_format();

      if self_s_i_f.0 == other_s_i_f.0 {
        None
      } else {
        let x_coord = (self_s_i_f.1 - other_s_i_f.1) / (self_s_i_f.0 - other_s_i_f.0);
        let y_coord = self_s_i_f.0 * x_coord + self_s_i_f.1;
        Some(Point {x: x_coord, y: y_coord})
      }
    }
  }

  pub fn calc_line_seg_intersect(
    &self,
    other_line: &DirectionalLine,
  ) -> Option<Point> {
    match self.calc_line_intersect(other_line) {
      Some(point) => {
        if self.point_is_on_line(&point) && other_line.point_is_on_line(&point) {
          Some(point)
        } else {
          None
        }
      }
      None => None
    }
  }

  pub fn get_vector_point(&self) -> Point {
    Point {
      x: self.end_point.x - self.start_point.x,
      y: self.end_point.y - self.start_point.y,
    }
  }

  pub fn draw(
    &self,
    ctx: &mut Context,
    screen: &ScreenContext,
  ) -> GameResult<()> {
    let mut mesh = graphics::MeshBuilder::new();
    mesh.line(
      &[screen.point_game_to_screen(self.start_point.into()), screen.point_game_to_screen(self.end_point.into())],
      3.0,
      [1.0, 1.0, 1.0, 1.0].into()
    )?;
    let built_mesh = mesh.build(ctx)?;
    graphics::draw(ctx, &built_mesh, graphics::DrawParam::default())
  }
}

#[derive(Deserialize)]
pub struct Zipper {
  line: DirectionalLine,
  width: f32,
  leading_dist: f32,
  pub strength: f32
}

#[allow(clippy::float_cmp)]
impl Zipper {
  pub fn new(
    p_start: &Point,
    p_end: &Point,
    width: f32,
    leading_dist: f32,
    strength: f32,
  ) -> Zipper {
    Zipper {
      line: DirectionalLine::new(p_start, p_end),
      width,
      leading_dist,
      strength,
    }
  }

  pub fn length(&self) -> f32 {
    self.line.length()
  }

  pub fn get_perp_through_point(
    &self,
    point: &Point
  ) -> DirectionalLine {
    if self.line.is_vertical() {
      DirectionalLine::new(point, &Point {x: self.line.start_point.x, y: point.y})
    } else if self.line.start_point.y == self.line.end_point.y {
      DirectionalLine::new(point, &Point {x: point.x, y: self.line.start_point.y})
    } else {
      let s_i_f = self.line.slope_int_format();

      let perp_slope = -1.0 / s_i_f.0;

      let perp_s_i_f = (perp_slope, -point.x * perp_slope + point.y);

      let perp_dir_line = DirectionalLine::new(
        &Point {x: 0.0,   y: perp_s_i_f.1},
        &Point {x: 100.0, y: perp_s_i_f.1 + 100.0 * perp_s_i_f.0},
      );

      DirectionalLine::new(
        point,
        &self.line.calc_line_intersect(&perp_dir_line).unwrap_or(Point {x: 0.0, y: 0.0})
      )
    }
  }

  pub fn advance_line(
    &self,
    line: &mut DirectionalLine
  ) {
    let mut point_to_advance = self.line.get_vector_point();
    if self.length() != 0.0 {
      point_to_advance.x *= self.leading_dist / self.length();
      point_to_advance.y *= self.leading_dist / self.length();
    }

    line.end_point.x += point_to_advance.x;
    line.end_point.y += point_to_advance.y;
  }

  pub fn point_in_range(
    &self,
    point: &Point,
  ) -> bool {
    let perpendicular_line = self.get_perp_through_point(point);
    self.line.point_is_on_line(&perpendicular_line.end_point) && perpendicular_line.length() < self.width
  }

  pub fn draw(
    &self,
    ctx: &mut Context,
    screen: &ScreenContext,
  ) -> GameResult<()> {
    self.line.draw(ctx, screen)
  }
}