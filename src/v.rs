//! vector
//!

pub mod v3;
pub mod v4;

use num::Float;

/// TVector
pub trait TVector<F: Float + std::fmt::Debug> {
  /// constructor
  fn new(v: &[F]) -> Self;
  /// like as slice
  fn me(&self) -> &[F];
  /// check equal with precision
  fn prec_eq(&self, e: F, v: &impl TVector<F>) -> bool;
  /// as_vec
  fn as_vec(&self) -> Vec<F>;
}
