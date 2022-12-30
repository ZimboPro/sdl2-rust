use std::ops;

use sdl2::rect::Rect;

pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  pub fn new(x: i32, y: i32) -> Self {
    Position { x, y }
  }

  pub fn set(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }

  pub fn update(&mut self, x: i32, y: i32) {
    self.x += x;
    self.y += y;
  }
}

impl ops::Add<Position> for Position {
  type Output = Position;
  fn add(self, rhs: Position) -> Self::Output {
    Position {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl ops::Sub<Position> for Position {
  type Output = Position;
  fn sub(self, rhs: Position) -> Self::Output {
    Position {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

pub trait PositionTrait {
  fn get_postion() -> &'static Position;
}

pub trait GetRect {
  fn rect_mut(&mut self) -> &mut Rect;
  fn rect(&self) -> &Rect;

  fn add_position(&mut self, x: i32, y: i32) {
    let r = self.rect_mut();
    r.x += x;
    r.y += y;
  }

  fn set_size(&mut self, width: u32, height: u32) {
    let r = self.rect_mut();
    r.set_width(width);
    r.set_height(height);
  }

  fn set_position(&mut self, x: i32, y: i32) {
    let t = self.rect_mut();
    t.set_x(x);
    t.set_y(y);
  }
}
