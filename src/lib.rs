#![doc(html_root_url = "https://docs.rs/quaternion-matrix/0.1.3")]
//! quaternion matrix for Rust
//!

pub mod q;
pub mod m;
pub mod v;

use num::Float;

/// check equal with precision
pub fn prec_eq_f<F: Float>(s: F, e: F, d: F) -> bool {
  (s - d).abs() < e
}

/// check equal with precision
pub fn prec_eq<F: Float>(s: &[F], e: F, d: &[F]) -> bool {
  for i in 0..s.len() { if !prec_eq_f(s[i], e, d[i]) { return false; } }
  true
}

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
  ($f: ident, $v: expr, $r: expr, $qe: expr) => {{
    let q = Quaternion::<$f>::from_axis_and_angle($v, $r);
    assert_pe!(q, 1e-6, $f, $qe);
    q
  }}
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
    assert_eq!(v32.dot(&v32), 14.0);
    assert_eq!(v64.dot(&v64), 14.0);
    let i32 = Matrix3::<f32>::identity();
    let i64 = Matrix3::<f64>::identity();
    assert_eq!(v32.dot_mv(&i32).to_vec(), v32.to_vec()); // i32 dot v32
    assert_eq!(v64.dot_mv(&i64).to_vec(), v64.to_vec()); // i64 dot v64
  }

  /// test Vector4
  #[test]
  fn test_vector4() {
    let v32 = Vector4::<f32>::new(&vec![1.0, 2.0, 3.0, 4.0]);
    let v64 = Vector4::<f64>::new(&vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(v32, [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(v64, [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(v32.dot(&v32), 30.0);
    assert_eq!(v64.dot(&v64), 30.0);
    let i32 = Matrix4::<f32>::identity();
    let i64 = Matrix4::<f64>::identity();
    assert_eq!(v32.dot_mv(&i32).to_vec(), v32.to_vec()); // i32 dot v32
    assert_eq!(v64.dot_mv(&i64).to_vec(), v64.to_vec()); // i64 dot v64
  }

  /// test Vector3 cross
  #[test]
  fn test_vector3_cross() {
    let a32 = Vector3::<f32>::new(&vec![1.0, 0.0, 1.0]);
    let a64 = Vector3::<f64>::new(&vec![1.0, 0.0, 1.0]);
    let b32 = Vector3::<f32>::new(&vec![0.0, 1.0, 1.0]);
    let b64 = Vector3::<f64>::new(&vec![0.0, 1.0, 1.0]);
    assert_eq!(a32.cross(&a32), [0.0, 0.0, 0.0]);
    assert_eq!(a64.cross(&a64), [0.0, 0.0, 0.0]);
    assert_eq!(a32.cross(&b32), [-1.0, -1.0, 1.0]);
    assert_eq!(a64.cross(&b64), [-1.0, -1.0, 1.0]);
    assert_eq!(b32.cross(&a32), [1.0, 1.0, -1.0]);
    assert_eq!(b64.cross(&a64), [1.0, 1.0, -1.0]);
  }

  /// test Vector4 cross (TODO: skip now)
  #[test]
  fn test_vector4_cross() {
/*
    let a32 = Vector4::<f32>::new(&vec![1.0, 0.0, 1.0, 1.0]);
    let a64 = Vector4::<f64>::new(&vec![1.0, 0.0, 1.0, 1.0]);
    let b32 = Vector4::<f32>::new(&vec![0.0, 1.0, 1.0, 1.0]);
    let b64 = Vector4::<f64>::new(&vec![0.0, 1.0, 1.0, 1.0]);
    assert_eq!(a32.cross(&a32), [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(a64.cross(&a64), [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(a32.cross(&b32), [-1.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0]);
    assert_eq!(a64.cross(&b64), [-1.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0]);
*/
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
    let q32p = Quaternion::<f32>::new(&vec![1.0, 0.0, 0.0, 1e-6]);
    let q64p = Quaternion::<f64>::new(&vec![1.0, 0.0, 0.0, 1e-6]);
    assert_eq!(q32, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(q64, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi32, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi64, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(q32, qi32);
    assert_eq!(q64, qi64);
    assert_eq!(qi32c, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi64c, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi32.prec_eq(1e-6, &q32m), true);
    assert_eq!(qi64.prec_eq(1e-6, &q64m), true);
    assert_eq!(qi32.prec_eq(1e-6, &q32p), false);
    assert_eq!(qi64.prec_eq(1e-6, &q64p), false);
    assert_eq!(qi32.to_vec(), [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(qi64.to_vec(), [1.0, 0.0, 0.0, 0.0]);
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
    // cos = r, sin = r
    // cos = 0, sin = 1
    // cos = -r, sin = r
    // cos = -1, sin = 0
    let q32t = vec![
      vec![
        check_qaa!(f32, &ax32, t32[0], &vec![1.0, 0.0, 0.0, 0.0]),
        check_qaa!(f32, &ax32, t32[1], &vec![r32, r32, 0.0, 0.0]),
        check_qaa!(f32, &ax32, t32[2], &vec![0.0, 1.0, 0.0, 0.0]),
        check_qaa!(f32, &ax32, t32[3], &vec![-r32, r32, 0.0, 0.0]),
        check_qaa!(f32, &ax32, t32[4], &vec![-1.0, 0.0, 0.0, 0.0])],
      vec![
        check_qaa!(f32, &ay32, t32[0], &vec![1.0, 0.0, 0.0, 0.0]),
        check_qaa!(f32, &ay32, t32[1], &vec![r32, 0.0, r32, 0.0]),
        check_qaa!(f32, &ay32, t32[2], &vec![0.0, 0.0, 1.0, 0.0]),
        check_qaa!(f32, &ay32, t32[3], &vec![-r32, 0.0, r32, 0.0]),
        check_qaa!(f32, &ay32, t32[4], &vec![-1.0, 0.0, 0.0, 0.0])],
      vec![
        check_qaa!(f32, &az32, t32[0], &vec![1.0, 0.0, 0.0, 0.0]),
        check_qaa!(f32, &az32, t32[1], &vec![r32, 0.0, 0.0, r32]),
        check_qaa!(f32, &az32, t32[2], &vec![0.0, 0.0, 0.0, 1.0]),
        check_qaa!(f32, &az32, t32[3], &vec![-r32, 0.0, 0.0, r32]),
        check_qaa!(f32, &az32, t32[4], &vec![-1.0, 0.0, 0.0, 0.0])]];
    let q64t = vec![
      vec![
        check_qaa!(f64, &ax64, t64[0], &vec![1.0, 0.0, 0.0, 0.0]),
        check_qaa!(f64, &ax64, t64[1], &vec![r64, r64, 0.0, 0.0]),
        check_qaa!(f64, &ax64, t64[2], &vec![0.0, 1.0, 0.0, 0.0]),
        check_qaa!(f64, &ax64, t64[3], &vec![-r64, r64, 0.0, 0.0]),
        check_qaa!(f64, &ax64, t64[4], &vec![-1.0, 0.0, 0.0, 0.0])],
      vec![
        check_qaa!(f64, &ay64, t64[0], &vec![1.0, 0.0, 0.0, 0.0]),
        check_qaa!(f64, &ay64, t64[1], &vec![r64, 0.0, r64, 0.0]),
        check_qaa!(f64, &ay64, t64[2], &vec![0.0, 0.0, 1.0, 0.0]),
        check_qaa!(f64, &ay64, t64[3], &vec![-r64, 0.0, r64, 0.0]),
        check_qaa!(f64, &ay64, t64[4], &vec![-1.0, 0.0, 0.0, 0.0])],
      vec![
        check_qaa!(f64, &az64, t64[0], &vec![1.0, 0.0, 0.0, 0.0]),
        check_qaa!(f64, &az64, t64[1], &vec![r64, 0.0, 0.0, r64]),
        check_qaa!(f64, &az64, t64[2], &vec![0.0, 0.0, 0.0, 1.0]),
        check_qaa!(f64, &az64, t64[3], &vec![-r64, 0.0, 0.0, r64]),
        check_qaa!(f64, &az64, t64[4], &vec![-1.0, 0.0, 0.0, 0.0])]];

    let i32 = Matrix4::<f32>::identity();
    let i64 = Matrix4::<f64>::identity();
    let qm32 = q32t.iter().map(|qxyz| qxyz.iter().map(|q|
      q.to_m4_rot()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let qm64 = q64t.iter().map(|qxyz| qxyz.iter().map(|q|
      q.to_m4_rot()).collect::<Vec<_>>()).collect::<Vec<_>>();

    assert!(qm32[0][0].prec_eq(1e-6, &i32));
    assert!(qm32[0][1].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, 0.0, -1.0, 0.0], // yz (0, -sin pi/2)
      vec![0.0, 1.0, 0.0, 0.0], // yz (sin pi/2, 0)
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[0][2].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, -1.0, 0.0, 0.0], // yz (cos pi, 0)
      vec![0.0, 0.0, -1.0, 0.0], // yz (0, cos pi)
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[0][3].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, 0.0, 1.0, 0.0], // yz (0, -sin 3pi/2)
      vec![0.0, -1.0, 0.0, 0.0], // yz (sin 3pi/2, 0)
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[0][4].prec_eq(1e-6, &i32));

    assert!(qm32[1][0].prec_eq(1e-6, &i32));
    assert!(qm32[1][1].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![0.0, 0.0, 1.0, 0.0], // zx (0, -sin pi/2)
      vec![0.0, 1.0, 0.0, 0.0],
      vec![-1.0, 0.0, 0.0, 0.0], // zx (sin pi/2, 0)
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[1][2].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![-1.0, 0.0, 0.0, 0.0], // zx (cos pi, 0)
      vec![0.0, 1.0, 0.0, 0.0],
      vec![0.0, 0.0, -1.0, 0.0], // zx (0, cos pi)
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[1][3].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![0.0, 0.0, -1.0, 0.0], // zx (0, -sin 3pi/2)
      vec![0.0, 1.0, 0.0, 0.0],
      vec![1.0, 0.0, 0.0, 0.0], // zx (sin 3pi/2, 0)
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[1][4].prec_eq(1e-6, &i32));

    assert!(qm32[2][0].prec_eq(1e-6, &i32));
    assert!(qm32[2][1].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![0.0, -1.0, 0.0, 0.0], // xy (0, -sin pi/2)
      vec![1.0, 0.0, 0.0, 0.0], // xy (sin pi/2, 0)
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[2][2].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![-1.0, 0.0, 0.0, 0.0], // xy (cos pi, 0)
      vec![0.0, -1.0, 0.0, 0.0], // xy (0, cos pi)
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[2][3].prec_eq(1e-6, &Matrix4::<f32>::new(&vec![
      vec![0.0, 1.0, 0.0, 0.0], // xy (0, -sin 3pi/2)
      vec![-1.0, 0.0, 0.0, 0.0], // xy (sin 3pi/2, 0)
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]])));
    assert!(qm32[2][4].prec_eq(1e-6, &i32));

    assert!(qm64[0][0].prec_eq(1e-6, &i64));
    assert!(qm64[0][4].prec_eq(1e-6, &i64));

    assert!(qm64[1][0].prec_eq(1e-6, &i64));
    assert!(qm64[1][4].prec_eq(1e-6, &i64));

    assert!(qm64[2][0].prec_eq(1e-6, &i64));
    assert!(qm64[2][4].prec_eq(1e-6, &i64));
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
    assert!(m32.dot_m(&m32).prec_eq(1e-6, &i32));
    assert!(m64.dot_m(&m64).prec_eq(1e-6, &i64));
    assert!(v32.dot_m(&u32).prec_eq(1e-6, &i32)); // u32 dot v32
    assert!(v64.dot_m(&u64).prec_eq(1e-6, &i64)); // u64 dot v64

    assert!(i32.inv(1e-6).expect("det").prec_eq(1e-6, &i32));
    assert!(i64.inv(1e-6).expect("det").prec_eq(1e-6, &i64));
    assert!(u32.inv(1e-6).expect("det").prec_eq(1e-6, &v32));
    assert!(u64.inv(1e-6).expect("det").prec_eq(1e-6, &v64));
    assert!(v32.inv(1e-6).expect("det").prec_eq(1e-6, &u32));
    assert!(v64.inv(1e-6).expect("det").prec_eq(1e-6, &u64));
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
      vec![0.5, -0.22727273, 0.36363637, -0.09090909],
      vec![0.5, -0.31818182, -0.09090909, 0.27272728],
      vec![-1.0, 0.45454547, 0.27272728, 0.18181819],
      vec![0.0, 0.27272728, -0.63636364, -0.09090909]]);
    let v64 = Matrix4::<f64>::new(&vec![
      vec![0.5, -0.22727272727, 0.36363636364, -0.09090909091],
      vec![0.5, -0.31818181818, -0.09090909091, 0.27272727273],
      vec![-1.0, 0.45454545455, 0.27272727273, 0.18181818182],
      vec![0.0, 0.27272727273, -0.63636363636, -0.09090909091]]);
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
    assert!(m32.dot_m(&m32).prec_eq(1e-6, &i32));
    assert!(m64.dot_m(&m64).prec_eq(1e-6, &i64));
    assert!(v32.dot_m(&u32).prec_eq(1e-6, &i32)); // u32 dot v32
    assert!(v64.dot_m(&u64).prec_eq(1e-6, &i64)); // u64 dot v64

    assert!(i32.inv(1e-6).expect("det").prec_eq(1e-6, &i32));
    assert!(i64.inv(1e-6).expect("det").prec_eq(1e-6, &i64));
    assert!(u32.inv(1e-6).expect("det").prec_eq(1e-6, &v32));
    assert!(u64.inv(1e-6).expect("det").prec_eq(1e-6, &v64));
    assert!(v32.inv(1e-6).expect("det").prec_eq(1e-6, &u32));
    assert!(v64.inv(1e-6).expect("det").prec_eq(1e-6, &u64));
  }
}
