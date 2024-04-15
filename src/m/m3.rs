//! matrix3
//!

use num::Float;

use crate::v::{TVector, v3::Vector3};
use crate::m::TMatrix;

/// Matrix3
pub type Matrix3<F> = [[F; 3]; 3];

/// TMatrix for Matrix3
impl<F: Float + std::fmt::Debug + std::iter::Sum> TMatrix<F> for Matrix3<F> {
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
    let m = m.me();
    for j in 0..3 {
      for i in 0..3 {
        if (self[j][i] - m[j][i]).abs() >= e { return false; }
      }
    }
    true
  }
  /// like as slice (TODO: now copy)
  fn me(&self) -> Vec<Vec<F>> {
    (0..3).into_iter().map(|j|
      (0..3).into_iter().map(|i|
        self[j][i]
      ).collect()
    ).collect()
  }
  /// dot m
  fn dot_m(a: &impl TMatrix<F>, b: &impl TMatrix<F>) -> Self {
    Matrix3::<F>::col_major(&(0..3).into_iter().map(|i|
      (0..3).into_iter().map(|j|
        Vector3::<F>::dot(&Vector3::<F>::new(&a.row(j)),
          &Vector3::<F>::new(&b.col(i)))
      ).collect()
    ).collect())
  }
  /// row (TODO: now copy)
  fn row(&self, j: usize) -> Vec<F> {
    (0..3).into_iter().map(|i| self[j][i]).collect()
  }
  /// col (TODO: now copy)
  fn col(&self, i: usize) -> Vec<F> {
    (0..3).into_iter().map(|j| self[j][i]).collect()
  }
}
