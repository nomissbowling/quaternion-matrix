//! matrix
//!

pub mod m3;
pub mod m4;

use num::Float;

use crate::v::{v3::Vector3, v4::Vector4};

/// cofactor
pub fn cofactor<F: Float + std::fmt::Debug>(
  m: &Vec<Vec<F>>, i: usize, j: usize) -> F {
/*
  assert_eq!(m.len(), m[0].len());
  assert!(m.len() >= 2);
*/
  if m.len() == 2 {
    return [[m[1][1]], [-m[1][0]], [-m[0][1]], [m[0][0]]][i][j];
  }
  let d = det(
    &m.iter().enumerate().flat_map(|(ri, r)|
      if i == ri { vec![] } // to be skipped by flatten
      else {
        vec![r.iter().enumerate().flat_map(|(cj, &c)|
          if j == cj { vec![] } // to be skipped by flatten
          else { vec![c] }
        ).collect::<Vec<_>>()]
      }
    ).collect::<Vec<_>>()
  );
  if (i + j) % 2 == 0 { d } else { -d }
}

/// transpose
pub fn transpose<F: Float + std::fmt::Debug>(m: &Vec<Vec<F>>) -> Vec<Vec<F>> {
  (0..m[0].len()).into_iter().map(|j|
    (0..m.len()).into_iter().map(|i|
      m[i][j]
    ).collect::<Vec<_>>()
  ).collect::<Vec<_>>()
}

/// det
pub fn det<F: Float + std::fmt::Debug>(m: &Vec<Vec<F>>) -> F {
/*
  assert_eq!(m.len(), m[0].len());
  assert!(m.len() >= 2);
*/
  if m.len() == 2 { return m[0][0] * m[1][1] - m[0][1] * m[1][0]; } // to fast
  let mut d = <F>::from(0).unwrap();
  for (cj, &c) in m[0].iter().enumerate() {
    d = d + c * cofactor(m, 0, cj); // += ops::AddAssign
  }
  d
}

/// inv
/// - TODO: recursive cofactor (slow), change to LU decomposition to be faster
/// - p: prec (assume det = 0)
pub fn inv<F: Float + std::fmt::Debug>(m: &Vec<Vec<F>>, p: F) ->
  Option<Vec<Vec<F>>> {
  assert_eq!(m.len(), m[0].len());
  assert!(m.len() >= 2);
  let d = det(m);
  if crate::prec_eq_f(d, p, <F>::from(0).unwrap()) { return None; }
  Some(transpose(&m.iter().enumerate().map(|(ri, r)|
    r.iter().enumerate().map(|(cj, _c)|
      cofactor(m, ri, cj) / d
    ).collect::<Vec<_>>()
  ).collect::<Vec<_>>()))
}

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
  /// to_vec
  fn to_vec(&self) -> Vec<Vec<F>>;
  /// transpose
  fn transpose(&self) -> Self where Self: Sized {
    Self::col_major(&self.to_vec())
  }
  /// det
  fn det(&self) -> F {
    crate::m::det(&self.to_vec())
  }
  /// inv
  /// - p: prec (assume det = 0)
  fn inv(&self, p: F) -> Option<Self> where Self: Sized {
    match crate::m::inv(&self.to_vec(), p) {
    None => None,
    Some(m) => Some(Self::new(&m))
    }
  }
}
