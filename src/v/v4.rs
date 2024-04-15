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
  /// like as slice
  fn me(&self) -> &[F] {
    self
  }
  /// check equal with precision
  fn prec_eq(&self, e: F, v: &impl TVector<F>) -> bool {
    for i in 0..4 {
      if (self[i] - v.me()[i]).abs() >= e { return false; }
    }
    true
  }
  /// as_vec
  fn as_vec(&self) -> Vec<F> {
    self.iter().map(|&f| f).collect::<Vec<_>>()
  }
}
