//! vector3
//!

use num::Float;

use crate::v::TVector;

/// Vector3
pub type Vector3<F> = [F; 3];

/// TVector for Vector3
impl<F: Float + std::fmt::Debug> TVector<F> for Vector3<F> {
  /// constructor
  fn new(v: &[F]) -> Self {
    (0..3).into_iter().map(|i| v[i]).collect::<Vec<_>>().try_into().unwrap()
  }
}
