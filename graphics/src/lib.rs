pub extern crate sdl2;

pub mod actor;
pub mod position;
pub mod projectile;
pub mod resource_manager;
pub mod setup;
pub mod vector;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
