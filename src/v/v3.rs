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
  /// a dot self
  fn dot(&self, a: &impl TVector<F>) -> F {
    let a = a.me();
    (0..3).into_iter().map(|i| a[i] * self[i]).sum::<F>()
  }
  /// m dot self
  fn dot_mv(&self, m: &impl TMatrix<F>) -> Self {
    Self::new(&(0..3).into_iter().map(|j| self.dot(&m.rowv3(j))).collect())
  }
  /// self cross b
  fn cross(&self, b: &impl TVector<F>) -> Self {
    let a = self.me();
    let b = b.me();
    Self::new(&vec![
      a[1] * b[2] - a[2] * b[1],
      a[2] * b[0] - a[0] * b[2],
      a[0] * b[1] - a[1] * b[0]])
  }
}
