//! matrix3
//!

use num::Float;

use crate::v::{TVector, v3::Vector3};
use crate::m::TMatrix;

/// Matrix3
pub type Matrix3<F> = [[F; 3]; 3];

/// TMatrix for Matrix3
impl<F: Float + std::fmt::Debug + std::iter::Sum> TMatrix<F> for Matrix3<F> {
  /// constructor col major from v3 (move)
  fn colmajor3(m: Vec<Vector3<F>>) -> Self where Self: Sized {
    (0..3).into_iter().map(|i|
      (0..3).into_iter().map(|j|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
  /// constructor row major from v3 (move)
  fn rowmajor3(m: Vec<Vector3<F>>) -> Self where Self: Sized {
    (0..3).into_iter().map(|j|
      m[j]
    ).collect::<Vec<_>>().try_into().unwrap()
  }
  /// constructor col major
  fn col_major(m: &Vec<Vec<F>>) -> Self {
    (0..3).into_iter().map(|i|
      (0..3).into_iter().map(|j|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
  /// constructor row major
  fn row_major(m: &Vec<Vec<F>>) -> Self {
    (0..3).into_iter().map(|j|
      (0..3).into_iter().map(|i|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
  /// constructor row major
  fn new(m: &Vec<Vec<F>>) -> Self {
    Self::row_major(m)
  }
  /// constructor
  fn identity() -> Self {
    let o = <F>::from(0).unwrap();
    let l = <F>::from(1).unwrap();
    Self::new(&vec![
      vec![l, o, o],
      vec![o, l, o],
      vec![o, o, l]])
  }
  /// check equal with precision
  fn prec_eq(&self, e: F, m: &impl TMatrix<F>) -> bool {
    let m = m.mev3();
    for j in 0..3 {
      for i in 0..3 {
        if (self[j][i] - m[j][i]).abs() >= e { return false; }
      }
    }
    true
  }
  /// like as slice v3
  fn mev3(&self) -> &[Vector3<F>] {
    self
  }
  /// m dot self
  fn dot_m(&self, m: &impl TMatrix<F>) -> Self {
    Matrix3::<F>::colmajor3((0..3).into_iter().map(|i| {
      self.colv3(i).dot_mv(m) // m dot self.col
    }).collect())
  }
  /// row to v3
  fn rowv3(&self, j: usize) -> Vector3<F> {
    Vector3::<F>::new(&(0..3).into_iter().map(|i| self[j][i]).collect())
  }
  /// col to v3
  fn colv3(&self, i: usize) -> Vector3<F> {
    Vector3::<F>::new(&(0..3).into_iter().map(|j| self[j][i]).collect())
  }
}
