#![doc(html_root_url = "https://docs.rs/quaternion-matrix/0.0.3")]
//! quaternion matrix for Rust
//!

pub mod q;
pub mod m;
pub mod v;

/// preq_eq macro
#[macro_export]
macro_rules! prec_eq {
  ($qs: expr, $e: expr, $f: ident, $qe: expr) => {
    $qs.prec_eq($e, &Quaternion::<$f>::new($qe))
  }
}

/// assert_pe macro
#[macro_export]
macro_rules! assert_pe {
  ($qs: expr, $e: expr, $f: ident, $qe: expr) => {
    assert!(prec_eq!($qs, $e, $f, $qe))
  }
}

/// check_q macro
#[macro_export]
macro_rules! check_qaa {
  ($f: ident, $v: expr, $r: expr, $qe: expr) => {
    let q = Quaternion::<$f>::from_axis_and_angle($v, $r);
    assert_pe!(q, 0.000001, $f, $qe);
  }
}

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
    let v32 = Vector3::<f32>::new(&vec![1.0, 2.0, 3.0]);
    let v64 = Vector3::<f64>::new(&vec![1.0, 2.0, 3.0]);
    assert_eq!(v32, [1.0, 2.0, 3.0]);
    assert_eq!(v64, [1.0, 2.0, 3.0]);
    assert_eq!(Vector3::<f32>::dot(&v32, &v32), 14.0);
    assert_eq!(Vector3::<f64>::dot(&v64, &v64), 14.0);
    let i32 = Matrix3::<f32>::identity();
    let i64 = Matrix3::<f64>::identity();
    assert_eq!(Vector3::<f32>::dot_mv(&i32, &v32).as_vec(), v32.as_vec());
    assert_eq!(Vector3::<f64>::dot_mv(&i64, &v64).as_vec(), v64.as_vec());
  }

  /// test Vector4
  #[test]
  fn test_vector4() {
    let v32 = Vector4::<f32>::new(&vec![1.0, 2.0, 3.0, 4.0]);
    let v64 = Vector4::<f64>::new(&vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(v32, [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(v64, [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(Vector4::<f32>::dot(&v32, &v32), 30.0);
    assert_eq!(Vector4::<f64>::dot(&v64, &v64), 30.0);
    let i32 = Matrix4::<f32>::identity();
    let i64 = Matrix4::<f64>::identity();
    assert_eq!(Vector4::<f32>::dot_mv(&i32, &v32).as_vec(), v32.as_vec());
    assert_eq!(Vector4::<f64>::dot_mv(&i64, &v64).as_vec(), v64.as_vec());
  }

  /// test Quaternion
  #[test]
  fn test_quaternion() {
    let qi32 = Quaternion::<f32>::identity();
    let qi64 = Quaternion::<f64>::identity();
    let qi32c = qi32.conjugate();
    let qi64c = qi64.conjugate();
    let q32 = Quaternion::<f32>::new(&vec![1.0, 0.0, 0.0, 0.0]);
    let q64 = Quaternion::<f64>::new(&vec![1.0, 0.0, 0.0, 0.0]);
    let q32m = Quaternion::<f32>::new(&vec![1.0, 0.0, 0.0, 0.0000009]);
    let q64m = Quaternion::<f64>::new(&vec![1.0, 0.0, 0.0, 0.0000009]);
    let q32p = Quaternion::<f32>::new(&vec![1.0, 0.0, 0.0, 0.000001]);
    let q64p = Quaternion::<f64>::new(&vec![1.0, 0.0, 0.0, 0.000001]);
    assert_eq!(q32, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(q64, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi32, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi64, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(q32, qi32);
    assert_eq!(q64, qi64);
    assert_eq!(qi32c, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi64c, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi32.prec_eq(0.000001, &q32m), true);
    assert_eq!(qi64.prec_eq(0.000001, &q64m), true);
    assert_eq!(qi32.prec_eq(0.000001, &q32p), false);
    assert_eq!(qi64.prec_eq(0.000001, &q64p), false);
    assert_eq!(qi32.as_vec(), [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi64.as_vec(), [1.0, 0.0, 0.0, 0.0]);
  }

  /// test Quaternion rot
  #[test]
  fn test_quaternion_rot() {
    let pi32 = std::f32::consts::PI;
    let pi64 = std::f64::consts::PI;
    let t32 = (0..=4).into_iter().map(|i|
      i as f32 * pi32 / 2.0).collect::<Vec<_>>();
    let t64 = (0..=4).into_iter().map(|i|
      i as f64 * pi64 / 2.0).collect::<Vec<_>>();
    let ax32 = Vector3::<f32>::new(&vec![1.0, 0.0, 0.0]);
    let ay32 = Vector3::<f32>::new(&vec![0.0, 1.0, 0.0]);
    let az32 = Vector3::<f32>::new(&vec![0.0, 0.0, 1.0]);
    let ax64 = Vector3::<f64>::new(&vec![1.0, 0.0, 0.0]);
    let ay64 = Vector3::<f64>::new(&vec![0.0, 1.0, 0.0]);
    let az64 = Vector3::<f64>::new(&vec![0.0, 0.0, 1.0]);
    let r32 = 0.70710678f32;
    let r64 = 0.70710678f64;

    // cos = 1, sin = 0
    check_qaa!(f32, &ax32, t32[0], &vec![1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f64, &ax64, t64[0], &vec![1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f32, &ay32, t32[0], &vec![1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f64, &ay64, t64[0], &vec![1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f32, &az32, t32[0], &vec![1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f64, &az64, t64[0], &vec![1.0, 0.0, 0.0, 0.0]);

    // cos = r, sin = r
    check_qaa!(f32, &ax32, t32[1], &vec![r32, r32, 0.0, 0.0]);
    check_qaa!(f64, &ax64, t64[1], &vec![r64, r64, 0.0, 0.0]);
    check_qaa!(f32, &ay32, t32[1], &vec![r32, 0.0, r32, 0.0]);
    check_qaa!(f64, &ay64, t64[1], &vec![r64, 0.0, r64, 0.0]);
    check_qaa!(f32, &az32, t32[1], &vec![r32, 0.0, 0.0, r32]);
    check_qaa!(f64, &az64, t64[1], &vec![r64, 0.0, 0.0, r64]);

    // cos = 0, sin = 1
    check_qaa!(f32, &ax32, t32[2], &vec![0.0, 1.0, 0.0, 0.0]);
    check_qaa!(f64, &ax64, t64[2], &vec![0.0, 1.0, 0.0, 0.0]);
    check_qaa!(f32, &ay32, t32[2], &vec![0.0, 0.0, 1.0, 0.0]);
    check_qaa!(f64, &ay64, t64[2], &vec![0.0, 0.0, 1.0, 0.0]);
    check_qaa!(f32, &az32, t32[2], &vec![0.0, 0.0, 0.0, 1.0]);
    check_qaa!(f64, &az64, t64[2], &vec![0.0, 0.0, 0.0, 1.0]);

    // cos = -r, sin = r
    check_qaa!(f32, &ax32, t32[3], &vec![-r32, r32, 0.0, 0.0]);
    check_qaa!(f64, &ax64, t64[3], &vec![-r64, r64, 0.0, 0.0]);
    check_qaa!(f32, &ay32, t32[3], &vec![-r32, 0.0, r32, 0.0]);
    check_qaa!(f64, &ay64, t64[3], &vec![-r64, 0.0, r64, 0.0]);
    check_qaa!(f32, &az32, t32[3], &vec![-r32, 0.0, 0.0, r32]);
    check_qaa!(f64, &az64, t64[3], &vec![-r64, 0.0, 0.0, r64]);

    // cos = -1, sin = 0
    check_qaa!(f32, &ax32, t32[4], &vec![-1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f64, &ax64, t64[4], &vec![-1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f32, &ay32, t32[4], &vec![-1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f64, &ay64, t64[4], &vec![-1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f32, &az32, t32[4], &vec![-1.0, 0.0, 0.0, 0.0]);
    check_qaa!(f64, &az64, t64[4], &vec![-1.0, 0.0, 0.0, 0.0]);
  }

  /// test Matrix3
  #[test]
  fn test_matrix3() {
    let i32 = Matrix3::<f32>::identity();
    let i64 = Matrix3::<f64>::identity();
    let m32 = Matrix3::<f32>::new(&vec![
      vec![1.0, 0.0, 0.0],
      vec![0.0, 1.0, 0.0],
      vec![0.0, 0.0, 1.0]]);
    let m64 = Matrix3::<f64>::new(&vec![
      vec![1.0, 0.0, 0.0],
      vec![0.0, 1.0, 0.0],
      vec![0.0, 0.0, 1.0]]);
    let u32 = Matrix3::<f32>::new(&vec![
      vec![1.0, 2.0, 1.0],
      vec![2.0, 1.0, 0.0],
      vec![1.0, 1.0, 2.0]]);
    let u64 = Matrix3::<f64>::new(&vec![
      vec![1.0, 2.0, 1.0],
      vec![2.0, 1.0, 0.0],
      vec![1.0, 1.0, 2.0]]);
    let v32 = Matrix3::<f32>::new(&vec![
      vec![-0.4, 0.6, 0.2],
      vec![0.8, -0.2, -0.4],
      vec![-0.2, -0.2, 0.6]]);
    let v64 = Matrix3::<f64>::new(&vec![
      vec![-0.4, 0.6, 0.2],
      vec![0.8, -0.2, -0.4],
      vec![-0.2, -0.2, 0.6]]);
    assert_eq!(m32, [
      [1.0, 0.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 0.0, 1.0]]);
    assert_eq!(m64, [
      [1.0, 0.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 0.0, 1.0]]);
    assert_eq!(m32, i32);
    assert_eq!(m64, i64);
    assert!(Matrix3::<f32>::dot_m(&m32, &m32).prec_eq(0.000001, &i32));
    assert!(Matrix3::<f64>::dot_m(&m64, &m64).prec_eq(0.000001, &i64));
    assert!(Matrix3::<f32>::dot_m(&u32, &v32).prec_eq(0.000001, &i32));
    assert!(Matrix3::<f64>::dot_m(&u64, &v64).prec_eq(0.000001, &i64));
  }

  /// test Matrix4
  #[test]
  fn test_matrix4() {
    let i32 = Matrix4::<f32>::identity();
    let i64 = Matrix4::<f64>::identity();
    let m32 = Matrix4::<f32>::new(&vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, 1.0, 0.0, 0.0],
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]]);
    let m64 = Matrix4::<f64>::new(&vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, 1.0, 0.0, 0.0],
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]]);
    let u32 = Matrix4::<f32>::new(&vec![
      vec![3.0, 1.0, 1.0, 2.0],
      vec![5.0, 1.0, 3.0, 4.0],
      vec![2.0, 0.0, 1.0, 0.0],
      vec![1.0, 3.0, 2.0, 1.0]]);
    let u64 = Matrix4::<f64>::new(&vec![
      vec![3.0, 1.0, 1.0, 2.0],
      vec![5.0, 1.0, 3.0, 4.0],
      vec![2.0, 0.0, 1.0, 0.0],
      vec![1.0, 3.0, 2.0, 1.0]]);
    let v32 = Matrix4::<f32>::new(&vec![
      vec![0.5, -0.2272727, 0.3636364, -0.0909091],
      vec![0.5, -0.3181818, -0.0909091, 0.2727273],
      vec![-1.0, 0.4545455, 0.2727273, 0.1818182],
      vec![0.0, 0.2727273, -0.6363636, -0.0909091]]);
    let v64 = Matrix4::<f64>::new(&vec![
      vec![0.5, -0.2272727, 0.3636364, -0.0909091],
      vec![0.5, -0.3181818, -0.0909091, 0.2727273],
      vec![-1.0, 0.4545455, 0.2727273, 0.1818182],
      vec![0.0, 0.2727273, -0.6363636, -0.0909091]]);
    assert_eq!(m32, [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]]);
    assert_eq!(m64, [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0]]);
    assert_eq!(m32, i32);
    assert_eq!(m64, i64);
    assert!(Matrix4::<f32>::dot_m(&m32, &m32).prec_eq(0.000001, &i32));
    assert!(Matrix4::<f64>::dot_m(&m64, &m64).prec_eq(0.000001, &i64));
    assert!(Matrix4::<f32>::dot_m(&u32, &v32).prec_eq(0.000001, &i32));
    assert!(Matrix4::<f64>::dot_m(&u64, &v64).prec_eq(0.000001, &i64));
  }
}
