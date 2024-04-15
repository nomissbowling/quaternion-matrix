//! matrix
//!

pub mod m3;
pub mod m4;

use num::Float;

/// TMatrix
pub trait TMatrix<F: Float + std::fmt::Debug> {
  /// constructor col major
  fn col_major(m: &Vec<Vec<F>>) -> Self;
  /// constructor row major
  fn row_major(m: &Vec<Vec<F>>) -> Self;
  /// constructor row major
  fn new(m: &Vec<Vec<F>>) -> Self;
  /// constructor
  fn identity() -> Self;
  /// check equal with precision
  fn prec_eq(&self, e: F, m: &impl TMatrix<F>) -> bool;
  /// like as slice (TODO: now copy)
  fn me(&self) -> Vec<Vec<F>>;
  /// dot m
  fn dot_m(a: &impl TMatrix<F>, b: &impl TMatrix<F>) -> Self;
  /// row (TODO: now copy)
  fn row(&self, j: usize) -> Vec<F>;
  /// col (TODO: now copy)
  fn col(&self, i: usize) -> Vec<F>;
}
