//! matrix
//!

pub mod m3;
pub mod m4;

use num::Float;

use crate::v::{v3::Vector3, v4::Vector4};

/// TMatrix
pub trait TMatrix<F: Float + std::fmt::Debug> {
  /// constructor col major from v3 (move)
  fn colmajor3(_m: Vec<Vector3<F>>) -> Self where Self: Sized { panic!("cm3") }
  /// constructor row major from v3 (move)
  fn rowmajor3(_m: Vec<Vector3<F>>) -> Self where Self: Sized { panic!("rm3") }
  /// constructor col major from v4 (move)
  fn colmajor4(_m: Vec<Vector4<F>>) -> Self where Self: Sized { panic!("cm4") }
  /// constructor row major from v4 (move)
  fn rowmajor4(_m: Vec<Vector4<F>>) -> Self where Self: Sized { panic!("rm4") }
  /// constructor col major
  fn col_major(m: &Vec<Vec<F>>) -> Self;
  /// constructor row major
  fn row_major(m: &Vec<Vec<F>>) -> Self;
  /// constructor row major
  fn new(m: &Vec<Vec<F>>) -> Self;
  /// constructor
  fn identity() -> Self;
  /// check equal with precision
  fn prec_eq(&self, e: F, m: &impl TMatrix<F>) -> bool;
  /// like as slice v3
  fn mev3(&self) -> &[Vector3<F>] { panic!("mev3") }
  /// like as slice v4
  fn mev4(&self) -> &[Vector4<F>] { panic!("mev4") }
  /// m dot self
  fn dot_m(&self, m: &impl TMatrix<F>) -> Self;
  /// row to v3
  fn rowv3(&self, _j: usize) -> Vector3<F> { panic!("rowv3") }
  /// col to v3
  fn colv3(&self, _i: usize) -> Vector3<F> { panic!("colv3") }
  /// row to v4
  fn rowv4(&self, _j: usize) -> Vector4<F> { panic!("rowv4") }
  /// col to v4
  fn colv4(&self, _i: usize) -> Vector4<F> { panic!("colv4") }
}
