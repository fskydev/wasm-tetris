use crate::shape::{Pos, Shape};
use std::{collections::HashSet, mem};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Left,
  Right,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Cell {
  Active = 1,
  Inactive = 0,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Tetris {
  width: i32,
  height: i32,
  current_shape: Shape,
  fixed_shapes: Vec<Shape>,
  cells: Vec<Cell>,
  lost: bool,
}

impl Tetris {
  pub fn is_out_of_bounds(&self, shape: &Shape) -> bool {
    !shape
      .iter_positions()
      .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
  }

  pub fn is_colliding(&self, shape: &Shape) -> bool {
    self
      .fixed_shapes
      .iter()
      .any(|fixed_shape| fixed_shape.collides_with(shape))
  }

  pub fn is_line_full(&self, y: i32) -> bool {
    self
      .fixed_shapes
      .iter()
      .flat_map(|shape| shape.iter_positions())
      .filter(|pos| pos.1 == y)
      .collect::<HashSet<_>>()
      .len() as i32
      == self.width
  }

  fn remove_line(&mut self, y: i32) {
    for shape in self.fixed_shapes.iter_mut() {
      shape.remove_line(y);
    }
  }

  fn remove_full_lines(&mut self) {
    for y in 0..self.height {
      if self.is_line_full(y) {
        self.remove_line(y);
      }
    }
  }

  fn get_index(&self, col: u32, row: u32) -> usize {
    (row * self.width as u32 + col) as usize
  }
}

#[wasm_bindgen]
impl Tetris {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width: width as i32,
      height: height as i32,
      current_shape: &Shape::new_random() + Pos((width as i32) / 2, 0), //todo!(),
      fixed_shapes: vec![],
      lost: false,
      cells: (0..width * height).map(|_| Cell::Inactive).collect(),     // cells: vec![],
    }
  }

  pub fn tick(&mut self) {
    if self.lost {
      return;
    }

    let translated_current_shape = &self.current_shape + Pos(0, 1);

    if self.is_out_of_bounds(&translated_current_shape)
      || self.is_colliding(&translated_current_shape)
    {
      let new_fixed_shape = mem::replace(
        &mut self.current_shape,
        &Shape::new_random() + Pos(self.width / 2, 0),
      );

      self.fixed_shapes.push(new_fixed_shape);
      self.remove_full_lines();

      if self.is_colliding(&self.current_shape) {
        self.lost = true;
      }
    } else {
      self.current_shape = translated_current_shape;
    }
  }

  pub fn shift(&mut self, direction: Direction) {
    if self.lost {
      return;
    }

    let translated_current_shape = &self.current_shape
      + match direction {
        Direction::Left => Pos(-1, 0),
        Direction::Right => Pos(1, 0),
      };

    if !self.is_out_of_bounds(&translated_current_shape)
      && !self.is_colliding(&translated_current_shape)
    {
      self.current_shape = translated_current_shape;
    }
  }

  pub fn rotate(&mut self) {
    if self.lost {
      return;
    }

    let rotated_current_shape = self.current_shape.rotated();

    if !self.is_out_of_bounds(&rotated_current_shape) && !self.is_colliding(&rotated_current_shape)
    {
      self.current_shape = rotated_current_shape;
    }
  }

  pub fn draw(&mut self) -> *const Cell {
    for col in 0..self.width {
      for row in 0..self.height {
        let idx = self.get_index(col as u32, row as u32);
        self.cells[idx] = Cell::Inactive;
      }
    }

    for pos in self.current_shape.iter_positions() {
      let idx = self.get_index(pos.0 as u32, pos.1 as u32);
      self.cells[idx] = Cell::Active;
    }

    for shape in self.fixed_shapes.iter_mut() {
      for pos in shape.iter_positions() {
        let idx = (pos.1 * self.width + pos.0) as usize;
        self.cells[idx] = Cell::Active;
      }
    }

    self.cells.as_ptr()
  }
}

#[cfg(test)]
mod tests {
  use super::Tetris;

  #[test]
  fn test() {
    let mut tetris = Tetris::new(10, 30);
    tetris.tick();
    tetris.tick();

    println!("{:#?}", tetris);
  }
}
