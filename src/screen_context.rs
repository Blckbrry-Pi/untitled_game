type VectorPoint = mint::Point2<f32>;

#[derive(Debug)]
pub struct ScreenContext {
  pub translation: mint::Point2<f32>,
  pub scale: f32,
}

impl ScreenContext {
  pub fn point_game_to_screen(
    &self,
    point: VectorPoint
  ) -> VectorPoint {
    VectorPoint {
      x: (point.x - self.translation.x) * self.scale,
      y: (point.y - self.translation.y) * self.scale,
    }
  }

  pub fn point_screen_to_game(
    &self,
    point: VectorPoint
  ) -> VectorPoint {
    VectorPoint {
      x: point.x / self.scale + self.translation.x,
      y: point.y / self.scale + self.translation.y,
    }
  }

  pub fn size_game_to_screen(
    &self,
    size: f32
  ) -> f32 {
    size * self.scale
  }

  pub fn size_screen_to_game(
    &self,
    size: f32
  ) -> f32 {
    size / self.scale
  }
}

impl Default for ScreenContext {
  fn default() -> ScreenContext {
    ScreenContext {
      translation: VectorPoint {
        x: 0.0,
        y: 0.0,
      },
      scale: 1.0
    }
  }
}