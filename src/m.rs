//! matrix
//!

pub mod m3;
pub mod m4;

use num::Float;

/// TMatrix
pub trait TMatrix<F: Float + std::fmt::Debug> {
  /// constructor
  fn new(m: &[&[F]]) -> Self;
}
