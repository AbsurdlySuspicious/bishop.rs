use std::ops::{Index, IndexMut};

#[derive(PartialEq, Debug)]
pub struct Vec2D<T> {
  pub vec: Vec<T>,
  pub w: usize,
  pub h: usize,
}

impl<T: Clone> Vec2D<T> {
  pub fn new(w: usize, h: usize, init: T) -> Vec2D<T> {
    Vec2D { vec: vec![init; w * h], w, h }
  }

  #[inline(always)]
  fn idx(&self, x: usize, y: usize) -> usize {
    (self.w * y) + x
  }

  pub fn get(&self, x: usize, y: usize) -> &T {
    &self.vec[self.idx(x, y)]
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
    let i = self.idx(x, y);
    &mut self.vec[i]
  }

  pub fn iget(&self, x: isize, y: isize) -> &T {
    assert!(x >= 0 && y >= 0);
    self.get(x as usize, y as usize)
  }

  pub fn iget_mut(&mut self, x: isize, y: isize) -> &mut T {
    assert!(x >= 0 && y >= 0);
    self.get_mut(x as usize, y as usize)
  }

  // pub fn get_row(&self, y: usize) -> &Vec<T> {
  //   &self.0[y]
  // }

  // pub fn into_vec(self) -> InnerVec<T> {
  //   self.0
  // }

  // pub fn vec(&self) -> &InnerVec<T> {
  //   &self.0
  // }
}

impl<T: Clone + Default> Vec2D<T> {
  pub fn new_default(w: usize, h: usize) -> Vec2D<T> {
    //<Vec2D<T>>::
    Self::new(w, h, Default::default())
  }
}

impl<T: Clone> Index<(usize, usize)> for Vec2D<T> {
  type Output = T;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    self.get(x, y)
  }
}

impl<T: Clone> Index<(isize, isize)> for Vec2D<T> {
  type Output = T;

  fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
    self.iget(x, y)
  }
}

impl<T: Clone> IndexMut<(usize, usize)> for Vec2D<T> {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    self.get_mut(x, y)
  }
}

impl<T: Clone> IndexMut<(isize, isize)> for Vec2D<T> {
  fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
    self.iget_mut(x, y)
  }
}
