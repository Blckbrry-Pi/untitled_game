use ggez::{graphics};
use ggez::{Context, GameResult};

type Point = mint::Point2<f32>;
use super::super::screen_context::ScreenContext;
use super::super::drawing_helpers::fill_stroke::{Fill, Stroke};

pub struct Attractor {
  pos: Point,
  field_size: f32,
  phys_size: f32,
  rot_offset: f32,
  spin_mult: f32,
}

impl Attractor {
  pub fn new(
    pos: Point,
    field_radius: f32,
    physical_radius: f32,
    spin_clockwise: bool,
  ) -> Attractor {
    Attractor {
      pos: pos.clone(),
      field_size: field_radius,
      phys_size: physical_radius,
      rot_offset: 0.0,
      spin_mult: if spin_clockwise {1.0} else {-1.0}
    }
  }

  pub fn in_range(
    &self,
    point: Point
  ) -> bool {
    let dist = ((self.pos.x - point.x).powf(2.0) + (self.pos.y - point.y).powf(2.0)).sqrt();
    dist < self.field_size
  }

  pub fn collided(
    &self,
    point: Point
  ) -> bool {
    let dist = ((self.pos.x - point.x).powf(2.0) + (self.pos.y - point.y).powf(2.0)).sqrt();
    dist < self.phys_size
  }

  pub fn rotate(
    &mut self,
    time_mult: f32
  ) {
    let degree_increment = 0.1 * time_mult * self.spin_mult;

    let radian_increment = degree_increment / 360.0 * std::f32::consts::TAU;

    self.rot_offset += radian_increment;
  }

  pub fn draw(
    &self,
    ctx: &mut Context,
    screen: &ScreenContext,
  ) -> GameResult<()> {
    let on_screen_point = screen.point_game_to_screen(self.pos);

    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      on_screen_point,
      self.field_size,
      0.5,
      [0.0, 1.0, 0.0, 0.157].into()
    )?;


    let dashed_stroke = (
      graphics::StrokeOptions::default().with_line_width(3.0),
      [0.0, 1.0, 0.0, 0.275].into()
    );

    let dashed_circle = dashed_circle_stroke(
      ctx,
      dashed_stroke,
      on_screen_point,
      self.field_size,
      30,
      self.rot_offset,
    )?;
    

    let spike_stroke = (
      graphics::StrokeOptions::default().with_line_width(1.0),
      [0.784, 0.784, 0.784, 1.0].into()
    );

    let spike_fill = (
      graphics::FillOptions::default(),
      [0.392, 0.392, 0.392, 1.0].into()
    );

    let inner_spike = spiky_circle(
      ctx,
      spike_stroke, 
      spike_fill,
      on_screen_point,
      self.phys_size * 2.0 / 3.0,
      self.phys_size,
      8,
      -self.rot_offset
    )?;

    graphics::draw(ctx, &circle,        graphics::DrawParam::default())?;
    graphics::draw(ctx, &dashed_circle, graphics::DrawParam::default())?;
    graphics::draw(ctx, &inner_spike,   graphics::DrawParam::default())?;

    Ok(()) 
  }
}

fn dashed_circle_stroke(
  ctx: &mut Context,
  stroke: Stroke,
  point: mint::Point2<f32>,
  radius: f32,
  steps: u16,
  rotation: f32
) -> GameResult<graphics::Mesh> {
  let mut mesh = graphics::MeshBuilder::new();

  for i in 0..steps {
      let mut dash_degrees = Vec::new();

      let mut radians = i as f64 / steps as f64 * std::f64::consts::TAU + rotation as f64;

      for _ in 0..50 {
          dash_degrees.push(radians);
          radians += 1.0 / steps as f64 / 100.0 * std::f64::consts::TAU;
      }


      let mut dash_points = Vec::new();

      for degree in dash_degrees.iter() {
          let x_pos = degree.cos() as f32 * radius + point.x;
          let y_pos = degree.sin() as f32 * radius + point.y;
          dash_points.push(mint::Point2 {x: x_pos, y: y_pos})
      }

      mesh.polyline(
          graphics::DrawMode::Stroke(stroke.0),
          &dash_points,
          stroke.1
      )?;
  }

  mesh.build(ctx)
}

fn spiky_circle(
  ctx: &mut Context,
  stroke: Stroke,
  fill: Fill,
  point: mint::Point2<f32>,
  inner_radius: f32,
  outer_radius: f32,
  spike_count: u16,
  rotation: f32
) -> GameResult<graphics::Mesh> {
  let mut mesh = graphics::MeshBuilder::new();
  
  for draw_mode_index in 0..2 {
    let mut points = Vec::new();
    for i in 0..spike_count {
      for j in 0..2 {
        let ang = (i as f32 + (j as f32 * 0.5)) / spike_count as f32 * std::f32::consts::TAU + rotation;
        let x_pos = ang.cos() * [inner_radius, outer_radius][j] + point.x;
        let y_pos = ang.sin() * [inner_radius, outer_radius][j] + point.y;
        points.push(mint::Point2 {x: x_pos, y: y_pos});
      }
    }
    let first_point;
    match points.first() {
      Some(p) => first_point = *p,
      None => first_point = mint::Point2 {x: 0.0, y: 0.0}
    }
    points.push(first_point);

    mesh.polyline(
      [graphics::DrawMode::Fill(fill.0), graphics::DrawMode::Stroke(stroke.0)][draw_mode_index],
      &points,
      [fill.1, stroke.1][draw_mode_index]
    )?;
  }

  mesh.build(ctx)
}