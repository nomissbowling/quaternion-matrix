//! vector
//!

pub mod v3;
pub mod v4;

use num::Float;

/// TVector
pub trait TVector<F: Float + std::fmt::Debug> {
  /// constructor
  fn new(v: &[F]) -> Self;
}
