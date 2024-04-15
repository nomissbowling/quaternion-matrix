//! vector4
//!

use num::Float;

use crate::v::TVector;

/// Vector4
pub type Vector4<F> = [F; 4];

/// TVector for Vector4
impl<F: Float + std::fmt::Debug> TVector<F> for Vector4<F> {
  /// constructor
  fn new(v: &[F]) -> Self {
    (0..4).into_iter().map(|i| v[i]).collect::<Vec<_>>().try_into().unwrap()
  }
}
