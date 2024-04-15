//! matrix3
//!

use num::Float;

use crate::m::TMatrix;

/// Matrix3
pub type Matrix3<F> = [[F; 3]; 3];

/// TMatrix for Matrix3
impl<F: Float + std::fmt::Debug> TMatrix<F> for Matrix3<F> {
  /// constructor
  fn new(m: &[&[F]]) -> Self {
    (0..3).into_iter().map(|j|
      (0..3).into_iter().map(|i|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
}
