use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl From<mint::Point2<f32>> for Point {
  fn from(item: mint::Point2<f32>) -> Self {
    Point {
      x: item.x,
      y: item.y,
    }
  }
}

impl From<Point> for mint::Point2<f32> {
  fn from(item: Point) -> Self {
    mint::Point2 {
      x: item.x,
      y: item.y,
    }
  }
}