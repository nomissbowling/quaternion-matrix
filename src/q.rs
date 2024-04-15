//! quaternion
//!

use num::Float;

use crate::v::{TVector, v3::Vector3};
use crate::m::{TMatrix, m4::{TM4, Matrix4}};

/// TQuaternion
pub trait TQuaternion<F: Float + std::fmt::Debug> {
  /// constructor
  fn from_axis_and_angle(v: &Vector3<F>, a: F) -> Self;
  /// constructor
  fn identity() -> Self;
  /// conjugate
  fn conjugate(&self) -> Self;
  /// like as ref
  fn me(&self) -> &Quaternion<F>;
  /// to Matrix4 (qp = Q4x4 p4)
  fn to_m4_left(&self) -> Matrix4<F>;
  /// to Matrix4 (qp = P4x4 q4)
  fn to_m4_right(&self) -> Matrix4<F>;
  /// to Matrix4 rot
  /// - q * r * q.conjugate()
  /// - Matrix4::from_p_right(q.conjugate()) * Matrix4::from_q_left(q) * r
  fn to_m4_rot(&self) -> Matrix4<F>;
}

/// Quaternion
pub type Quaternion<F> = [F; 4];

/// TQuaternion for Quaternion
impl<F: Float + std::fmt::Debug + std::iter::Sum> TQuaternion<F> for Quaternion<F> {
  /// constructor
  fn from_axis_and_angle(v: &Vector3<F>, a: F) -> Self {
    let t = a / <F>::from(2.0).unwrap();
    let d = v.iter().map(|&p| p * p).sum::<F>().sqrt();
    let n = v.iter().map(|&p| p / d).collect::<Vec<_>>();
    [t.cos(), n[0] * t.sin(), n[1] * t.sin(), n[2] * t.sin()]
  }
  /// constructor
  fn identity() -> Self {
    Self::new(&(0..4).into_iter().map(|i|
      <F>::from(if i == 0 { 1.0 } else { 0.0 }).unwrap()
    ).collect::<Vec<_>>())
  }
  /// conjugate
  fn conjugate(&self) -> Self {
    [self[0], -self[1], -self[2], -self[3]]
  }
  /// like as ref
  fn me(&self) -> &Quaternion<F> {
    self
  }
  /// to Matrix4 (qp = Q4x4 p4)
  fn to_m4_left(&self) -> Matrix4<F> {
    Matrix4::<F>::from_q_left(self)
  }
  /// to Matrix4 (qp = P4x4 q4)
  fn to_m4_right(&self) -> Matrix4<F> {
    Matrix4::<F>::from_p_right(self)
  }
  /// to Matrix4 rot
  /// - q * r * q.conjugate()
  /// - Matrix4::from_p_right(q.conjugate()) * Matrix4::from_q_left(q) * r
  fn to_m4_rot(&self) -> Matrix4<F> {
    let m4q = Matrix4::from_q_left(self);
    let m4p = Matrix4::from_p_right(&self.conjugate());
    Matrix4::<F>::dot_m(&m4p, &m4q)
  }
}
