#![doc(html_root_url = "https://docs.rs/quaternion-matrix/0.0.2")]
//! quaternion matrix for Rust
//!

pub mod q;
pub mod m;
pub mod v;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;
  use crate::v::{TVector, v3::Vector3, v4::Vector4};
  use crate::q::{TQuaternion, Quaternion};
  use crate::m::{TMatrix, m3::Matrix3, m4::Matrix4};

  /// test Vector3
  #[test]
  fn test_vector3() {
    assert_eq!(Vector3::<f32>::new(
      &[1.0, 2.0, 3.0]),
      [1.0, 2.0, 3.0]);
    assert_eq!(Vector3::<f64>::new(
      &[1.0, 2.0, 3.0]),
      [1.0, 2.0, 3.0]);
  }

  /// test Vector4
  #[test]
  fn test_vector4() {
    assert_eq!(Vector4::<f32>::new(
      &[1.0, 2.0, 3.0, 4.0]),
      [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(Vector4::<f64>::new(
      &[1.0, 2.0, 3.0, 4.0]),
      [1.0, 2.0, 3.0, 4.0]);
  }

  /// test Quaternion
  #[test]
  fn test_quaternion() {
    assert_eq!(Quaternion::<f32>::new(
      &[1.0, 0.0, 0.0, 0.0]),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f64>::new(
      &[1.0, 0.0, 0.0, 0.0]),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f32>::identity(),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f64>::identity(),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f32>::identity().conjugate(),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f64>::identity().conjugate(),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f32>::identity().prec_eq(0.000001,
      &Quaternion::<f32>::new(&[1.0, 0.0, 0.0, 0.0000009])), true);
    assert_eq!(Quaternion::<f64>::identity().prec_eq(0.000001,
      &Quaternion::<f64>::new(&[1.0, 0.0, 0.0, 0.0000009])), true);
    assert_eq!(Quaternion::<f32>::identity().prec_eq(0.000001,
      &Quaternion::<f32>::new(&[1.0, 0.0, 0.0, 0.000001])), false);
    assert_eq!(Quaternion::<f64>::identity().prec_eq(0.000001,
      &Quaternion::<f64>::new(&[1.0, 0.0, 0.0, 0.000001])), false);
    assert_eq!(Quaternion::<f32>::identity().as_vec(),
      [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(Quaternion::<f64>::identity().as_vec(),
      [1.0, 0.0, 0.0, 0.0]);
  }

  /// test Matrix3
  #[test]
  fn test_matrix3() {
    assert_eq!(Matrix3::<f32>::new(&[
      &[1.0, 0.0, 0.0],
      &[0.0, 1.0, 0.0],
      &[0.0, 0.0, 1.0]]), [
      [1.0, 0.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 0.0, 1.0]]);
    assert_eq!(Matrix3::<f64>::new(&[
      &[1.0, 0.0, 0.0],
      &[0.0, 1.0, 0.0],
      &[0.0, 0.0, 1.0]]), [
      [1.0, 0.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 0.0, 1.0]]);
  }

  /// test Matrix4
  #[test]
  fn test_matrix4() {
    assert_eq!(Matrix4::<f32>::new(&[
      &[1.0, 0.0, 0.0, 0.0],
      &[0.0, 1.0, 0.0, 0.0],
      &[0.0, 0.0, 1.0, 0.0],
      &[0.0, 0.0, 0.0, 1.0]]), [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]]);
    assert_eq!(Matrix4::<f64>::new(&[
      &[1.0, 0.0, 0.0, 0.0],
      &[0.0, 1.0, 0.0, 0.0],
      &[0.0, 0.0, 1.0, 0.0],
      &[0.0, 0.0, 0.0, 1.0]]), [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]]);
  }
}
