//! matrix4
//!

use num::Float;

use crate::v::{TVector, v4::Vector4};
use crate::m::TMatrix;
use crate::q::TQuaternion;

/// TM4
pub trait TM4<F: Float + std::fmt::Debug> {
  /// from Quaternion (qp = Q4x4 p4)
  fn from_q_left(q: &impl TQuaternion<F>) -> Self;
  /// from Quaternion (qp = P4x4 q4)
  fn from_p_right(p: &impl TQuaternion<F>) -> Self;
  /// from Quaternion rot (qrp = P4x4 Q4x4 r4)
  fn from_rot(q: &impl TQuaternion<F>) -> Self;
}

/// Matrix4
pub type Matrix4<F> = [[F; 4]; 4];

/// TMatrix for Matrix4
impl<F: Float + std::fmt::Debug + std::iter::Sum> TMatrix<F> for Matrix4<F> {
  /// constructor col major
  fn col_major(m: &Vec<Vec<F>>) -> Self {
    (0..4).into_iter().map(|i|
      (0..4).into_iter().map(|j|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
  /// constructor row major
  fn row_major(m: &Vec<Vec<F>>) -> Self {
    (0..4).into_iter().map(|j|
      (0..4).into_iter().map(|i|
        m[j][i]
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap()
  }
  /// constructor row major
  fn new(m: &Vec<Vec<F>>) -> Self {
    Self::row_major(m)
  }
  /// constructor
  fn identity() -> Self {
    let o = <F>::from(0).unwrap();
    let l = <F>::from(1).unwrap();
    Self::new(&vec![
      vec![l, o, o, o],
      vec![o, l, o, o],
      vec![o, o, l, o],
      vec![o, o, o, l]])
  }
  /// check equal with precision
  fn prec_eq(&self, e: F, m: &impl TMatrix<F>) -> bool {
    let m = m.me();
    for j in 0..4 {
      for i in 0..4 {
        if (self[j][i] - m[j][i]).abs() >= e { return false; }
      }
    }
    true
  }
  /// like as slice (TODO: now copy)
  fn me(&self) -> Vec<Vec<F>> {
    (0..4).into_iter().map(|j|
      (0..4).into_iter().map(|i|
        self[j][i]
      ).collect()
    ).collect()
  }
  /// dot m
  fn dot_m(a: &impl TMatrix<F>, b: &impl TMatrix<F>) -> Self {
    Matrix4::<F>::col_major(&(0..4).into_iter().map(|i| {
      let v = Vector4::<F>::new(&b.col(i));
      Vector4::<F>::dot_mv(a, &v).as_vec()
/*
      (0..4).into_iter().map(|j|
        Vector4::<F>::dot(&Vector4::<F>::new(&a.row(j)),
          &Vector4::<F>::new(&b.col(i)))
      ).collect()
*/
    }).collect())
  }
  /// row (TODO: now copy)
  fn row(&self, j: usize) -> Vec<F> {
    (0..4).into_iter().map(|i| self[j][i]).collect()
  }
  /// col (TODO: now copy)
  fn col(&self, i: usize) -> Vec<F> {
    (0..4).into_iter().map(|j| self[j][i]).collect()
  }
}

/// TM4 for Matrix4
impl<F: Float + std::fmt::Debug> TM4<F> for Matrix4<F> {
  /// from Quaternion (qp = Q4x4 p4)
  fn from_q_left(q: &impl TQuaternion<F>) -> Self {
    let q = q.me();
    [
      [q[0], -q[1], -q[2], -q[3]],
      [q[1], q[0], -q[3], q[2]],
      [q[2], q[3], q[0], -q[1]],
      [q[3], -q[2], q[1], q[0]]]
  }
  /// from Quaternion (qp = P4x4 q4)
  fn from_p_right(p: &impl TQuaternion<F>) -> Self {
    let p = p.me();
    [
      [p[0], -p[1], -p[2], -p[3]],
      [p[1], p[0], p[3], -p[2]],
      [p[2], -p[3], p[0], p[1]],
      [p[3], p[2], -p[1], p[0]]]
  }
  /// from Quaternion rot (qrp = P4x4 Q4x4 r4)
  fn from_rot(q: &impl TQuaternion<F>) -> Self {
    q.to_m4_rot()
  }
}
