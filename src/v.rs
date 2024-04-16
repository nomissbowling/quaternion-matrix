//! vector
//!

pub mod v3;
pub mod v4;

use num::Float;

use crate::m::TMatrix;

/// TVector
pub trait TVector<F: Float + std::fmt::Debug> {
  /// constructor
  fn new(v: &Vec<F>) -> Self;
  /// check equal with precision
  fn prec_eq(&self, e: F, v: &impl TVector<F>) -> bool;
  /// to_vec
  fn to_vec(&self) -> Vec<F>;
  /// like as slice
  fn me(&self) -> &[F];
  /// a dot self
  fn dot(&self, a: &impl TVector<F>) -> F;
  /// m dot self
  fn dot_mv(&self, m: &impl TMatrix<F>) -> Self;
}
