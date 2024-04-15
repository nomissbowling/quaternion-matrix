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
  /// as_vec (TODO: now copy)
  fn as_vec(&self) -> Vec<F>;
  /// like as slice
  fn me(&self) -> &[F];
  /// dot
  fn dot(a: &impl TVector<F>, b: &impl TVector<F>) -> F;
  /// dot mv
  fn dot_mv(a: &impl TMatrix<F>, v: &impl TVector<F>) -> Self;
}
