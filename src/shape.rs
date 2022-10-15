use std::{collections::HashSet, ops::Add};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos(self.0 + rhs.0, self.1 + rhs.1)
  }
}

#[derive(Debug, Clone)]
pub struct Shape {
  positions: HashSet<Pos>,
  anchor: Pos,
}

macro_rules! impl_shape_constructor {
  ($($new:ident: [$($pos:expr),*] anchored at $anchor:expr;)*) => {
    $(
      pub fn $new() -> Self {
        Self {
          positions: [$($pos),*].into_iter().collect(),
          anchor: $anchor,
        }
      }
    )*
  };
}

impl Shape {
  impl_shape_constructor! {
    new_i: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] anchored at Pos(1, 0);
    new_o: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] anchored at Pos(0, 0);
    new_t: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] anchored at Pos(1, 0);
    new_j: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] anchored at Pos(0, 1);
    new_l: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] anchored at Pos(0, 1);
    new_s: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] anchored at Pos(0, 0);
    new_z: [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] anchored at Pos(0, 0);
  }

  pub fn new_random() -> Self {
    let random = (rand::random::<f64>() * 7.0).floor() as u8;

    match random {
      0 => Self::new_i(),
      1 => Self::new_o(),
      2 => Self::new_t(),
      3 => Self::new_j(),
      4 => Self::new_l(),
      5 => Self::new_s(),
      6 => Self::new_z(),
      _ => unreachable!(),
    }
  }

  pub fn iter_positions(&self) -> impl Iterator<Item = Pos> + '_ {
    // bound to the self lifetime
    self.positions.iter().copied()
  }

  pub fn collides_with(&self, other: &Shape) -> bool {
    self.positions.intersection(&other.positions).count() > 0
  }

  pub fn rotated(&self) -> Self {
    let Pos(a, b) = self.anchor;

    Self {
      positions: self
        .iter_positions()
        .map(|Pos(x, y)| Pos(-y + b + a, x - a + b))
        .collect(),
      anchor: self.anchor,
    }
  }

  pub fn remove_line(&mut self, y: i32) {
    self.positions = self
      .positions
      .iter()
      .copied()
      .filter(|pos| pos.1 != y)
      .map(|pos| {
        if pos.1 >= y {
          pos
        } else {
          Pos(pos.0, pos.1 + 1)
        }
      })
      .collect();
  }
}

impl Add<Pos> for &Shape {
  type Output = Shape;

  fn add(self, rhs: Pos) -> Self::Output {
    Shape {
      positions: self.positions.iter().map(|&pos| pos + rhs).collect(),
      anchor: self.anchor + rhs,
    }
  }
}
