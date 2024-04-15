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
  /// like as slice
  fn me(&self) -> &[F] {
    self
  }
  /// check equal with precision
  fn prec_eq(&self, e: F, v: &impl TVector<F>) -> bool {
    for i in 0..3 {
      if (self[i] - v.me()[i]).abs() >= e { return false; }
    }
    true
  }
  /// as_vec
  fn as_vec(&self) -> Vec<F> {
    self.iter().map(|&f| f).collect::<Vec<_>>()
  }
}
