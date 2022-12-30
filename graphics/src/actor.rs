use sdl2::{rect::Rect, render::WindowCanvas, video::WindowContext};

use crate::{position::GetRect, resource_manager::TextureManager, vector::Vector};

pub struct Actor {
  texture_path: String,
  rect: Rect,
  angle: f64,
  pub vector: Vector,
  max: f32,
}

impl Actor {
  pub fn new(
    resource_manager: &mut TextureManager<WindowContext>,
    image_path: &str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
  ) -> Actor {
    let _t = resource_manager.load(image_path).unwrap();
    Actor {
      texture_path: image_path.to_string(),
      rect: Rect::new(x, y, width, height),
      angle: 0.,
      vector: Vector::new(0., 0.),
      max: 5.,
    }
  }

  pub fn draw(
    &self,
    canvas: &mut WindowCanvas,
    resource_manager: &mut TextureManager<WindowContext>,
  ) {
    let texture = resource_manager.load(&self.texture_path).unwrap();
    canvas
      .copy_ex(&texture, None, self.rect, self.angle, None, false, false)
      .unwrap();
  }

  pub fn get_angle(&self) -> f64 {
    self.angle
  }

  pub fn set_angle(&mut self, angle: f64) {
    self.angle = angle;
  }

  pub fn update_angle(&mut self, angle: f64) {
    self.angle += angle;
  }

  pub fn get_x(&self) -> i32 {
    self.rect.x
  }

  pub fn set_x(&mut self, x: i32) {
    self.rect.x = x;
  }

  pub fn update_x(&mut self, x: i32) {
    self.rect.x += x;
  }

  pub fn get_y(&self) -> i32 {
    self.rect.y
  }

  pub fn set_y(&mut self, y: i32) {
    self.rect.y = y;
  }

  pub fn update_y(&mut self, y: i32) {
    self.rect.y += y;
  }

  pub fn get_width(&self) -> i32 {
    self.rect.w
  }

  pub fn set_width(&mut self, width: i32) {
    self.rect.w = width;
  }

  pub fn update_width(&mut self, width: i32) {
    self.rect.w += width;
  }

  pub fn get_height(&self) -> i32 {
    self.rect.h
  }

  pub fn set_height(&mut self, height: i32) {
    self.rect.h = height;
  }

  pub fn update_height(&mut self, height: i32) {
    self.rect.h += height;
  }

  pub fn update_pos(&mut self) {
    let t = self.vector.pixel_diff(self.max);
    if !t.is_zero() {
      t.update_rect(&mut self.rect);
    }
  }
}

impl GetRect for Actor {
  fn rect(&self) -> &Rect {
    &self.rect
  }

  fn rect_mut(&mut self) -> &mut Rect {
    &mut self.rect
  }
}
