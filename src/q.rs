//! quaternion
//!

use num::Float;

use crate::v::TVector;

/// TQuaternion
pub trait TQuaternion<F: Float + std::fmt::Debug> {
  /// constructor
  fn identity() -> Self;
}

/// Quaternion
pub type Quaternion<F> = [F; 4];

/// TQuaternion for Quaternion
impl<F: Float + std::fmt::Debug> TQuaternion<F> for Quaternion<F> {
  /// constructor
  fn identity() -> Self {
    Quaternion::<F>::new(&(0..4).into_iter().map(|i|
      <F>::from(if i == 0 { 1.0 } else { 0.0 }).unwrap()
    ).collect::<Vec<_>>())
  }
}
