use sdl2::rect::Rect;

pub struct Vector {
  pub x: f32,
  pub y: f32,
}

pub struct PixelDifference {
  pub x: i32,
  pub y: i32,
}

impl Vector {
  pub fn new(x: f32, y: f32) -> Self {
    let m = (x.powi(2) + y.powi(2)).sqrt();
    if m == 0. {
      return Vector { x: 0., y: 0. };
    }
    Vector { x: x / m, y: y / m }
  }

  pub fn update(&mut self) {
    let m = (self.x.powi(2) + self.y.powi(2)).sqrt();
    if m != 0. {
      self.x /= m;
      self.y /= m;
    }
  }

  pub fn set_x(&mut self, x: f32) {
    self.x = x;
    self.update();
  }

  pub fn set_y(&mut self, y: f32) {
    self.y = y;
    self.update();
  }

  pub fn set(&mut self, x: f32, y: f32) {
    self.x = x;
    self.y = y;
    self.update();
  }

  pub fn pixel_diff(&self, velocity: f32) -> PixelDifference {
    PixelDifference {
      x: (self.x * velocity) as i32,
      y: (self.y * velocity) as i32,
    }
  }
}

impl PixelDifference {
  pub fn update_rect(&self, rect: &mut Rect) {
    rect.x += self.x;
    rect.y += self.y;
  }

  pub fn is_zero(&self) -> bool {
    self.x == 0 && self.y == 0
  }
}

pub trait VectorTrait {
  fn vector() -> &'static Vector;
}
