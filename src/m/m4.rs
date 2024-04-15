//! matrix4
//!

use num::Float;

use crate::m::TMatrix;

/// Matrix4
pub type Matrix4<F> = [[F; 4]; 4];

/// TMatrix for Matrix4
impl<F: Float + std::fmt::Debug> TMatrix<F> for Matrix4<F> {
  /// constructor
  fn new(m: &[&[F]]) -> Self {
    (0..4).into_iter().map(|j|
      (0..4).into_iter().map(|i|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
}
