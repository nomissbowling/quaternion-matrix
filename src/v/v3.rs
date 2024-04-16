//! vector3
//!

use num::Float;

use crate::v::TVector;
use crate::m::TMatrix;

/// Vector3
pub type Vector3<F> = [F; 3];

/// TVector for Vector3
impl<F: Float + std::fmt::Debug + std::iter::Sum> TVector<F> for Vector3<F> {
  /// constructor
  fn new(v: &Vec<F>) -> Self {
    (0..3).into_iter().map(|i| v[i]).collect::<Vec<_>>().try_into().unwrap()
  }
  /// check equal with precision
  fn prec_eq(&self, e: F, v: &impl TVector<F>) -> bool {
    for i in 0..3 {
      if (self[i] - v.me()[i]).abs() >= e { return false; }
    }
    true
  }
  /// to_vec
  fn to_vec(&self) -> Vec<F> {
    self.iter().map(|&f| f).collect::<Vec<_>>()
  }
  /// like as slice
  fn me(&self) -> &[F] {
    self
  }
  /// dot
  fn dot(a: &impl TVector<F>, b: &impl TVector<F>) -> F {
    let a = a.me();
    let b = b.me();
    (0..3).into_iter().map(|i| a[i] * b[i]).sum::<F>()
  }
  /// dot mv
  fn dot_mv(a: &impl TMatrix<F>, v: &impl TVector<F>) -> Self {
    Self::new(&(0..3).into_iter().map(|j|
      Self::dot(&Self::new(&a.row(j)), v)).collect())
  }
}
