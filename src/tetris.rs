use crate::shape::{Pos, Shape};

#[derive(Debug)]
pub struct Tetris {
  width: u32,
  height: u32,
  current_shape: Shape,
  fixed_shapes: Vec<Shape>,
}

impl Tetris {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      current_shape: &Shape::new_random() + Pos((width as i32) / 2, 0),           //todo!(),
      fixed_shapes: vec![],
    }
  }

  pub fn tick(&mut self) {
    
  }
}

#[cfg(test)]
mod tests {
  use super::Tetris;

  #[test]
  fn test() {
    println!("{:#?}", Tetris::new(10, 30));
  }
}
