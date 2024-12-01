//! Square matrix of bits (Galois(2))
//!
//! The bits in the matrix are represented by an array of unsigned integers,
//! of a bit-width that is suitable for the desired matrix dimensions.
//! Eg for a 32×32 matrix, an array of u32 of length 32 is used.

use core::ops::{BitAnd, Shl, Shr};
use num_traits::{ConstOne, ConstZero, One, Pow, PrimInt, Unsigned, Zero};

/// Shorthand for traits needed in `BCMatrix`.
pub trait BitMatrixInt: PrimInt + Unsigned + ConstOne + ConstZero + core::ops::BitXorAssign {}
impl<T: PrimInt + Unsigned + ConstOne + ConstZero + core::ops::BitXorAssign> BitMatrixInt for T {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BCMatrix<T, const WIDTH: usize>
where
    T: BitMatrixInt,
{
    columns: [T; WIDTH],
}

impl<T, const WIDTH: usize> BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    pub fn new(init_data: &[T; WIDTH]) -> BCMatrix<T, WIDTH> {
        BCMatrix::<T, WIDTH> {
            columns: *init_data,
        }
    }

    pub fn shift(shift_value: i8) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH> {
            columns: [T::ZERO; WIDTH],
        };
        let mut value: T = if shift_value >= 0 {
            T::ONE << shift_value as usize
        } else {
            T::ZERO
        };
        let mask = crate::math::bit_width_mask(WIDTH);
        let mut shift_temp = shift_value;
        for i in 0..WIDTH {
            result.columns[i] = value;
            if shift_temp < 0 {
                shift_temp += 1;
                if shift_temp == 0 {
                    value = T::ONE;
                }
            } else {
                value = (value << 1) & mask;
            }
        }
        result
    }

    pub fn dot_vec(&self, b: T) -> T {
        let mut result: T = T::ZERO;
        let mut b_temp = b;
        for i in 0..WIDTH {
            if b_temp & T::ONE != T::ZERO {
                result ^= self.columns[i];
            }
            b_temp = b_temp >> 1;
        }
        result
    }

    pub fn dot(&self, b: &BCMatrix<T, WIDTH>) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.dot_vec(b.columns[i]);
        }
        result
    }

    pub fn dot_equ(&mut self, b: &BCMatrix<T, WIDTH>) {
        let a = BCMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            self.columns[i] = a.dot_vec(b.columns[i]);
        }
    }
}

impl<T, const WIDTH: usize> Zero for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    /// Create a zero-matrix.
    fn zero() -> BCMatrix<T, WIDTH> {
        BCMatrix::<T, WIDTH> {
            columns: [T::ZERO; WIDTH],
        }
    }

    fn is_zero(&self) -> bool {
        for i in 0..WIDTH {
            if self.columns[i] != T::ZERO {
                return false;
            }
        }
        true
    }
}

impl<T, const WIDTH: usize> One for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    /// Create a unity-matrix. That is, ones on the diagonal, zeros elsewhere.
    fn one() -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH> {
            columns: [T::ZERO; WIDTH],
        };
        let mut value: T = T::ONE;
        for i in 0..WIDTH {
            result.columns[i] = value;
            value = value << 1;
        }
        result
    }
}

impl<N, T, const WIDTH: usize> Pow<N> for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
    N: Unsigned + PrimInt + BitAnd + ConstOne + ConstZero,
{
    type Output = Self;

    /// Raise a matrix to a power. Efficient matrix exponentiation.
    fn pow(self, n: N) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH>::one();
        let mut temp_exp = BCMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        let mut n_work: N = n;

        loop {
            if n_work & N::ONE != N::ZERO {
                result.dot_equ(&temp_exp);
            }
            n_work = n_work >> 1;
            if n_work == N::ZERO {
                break;
            }
            let temp_exp2 = temp_exp.clone();
            temp_exp.dot_equ(&temp_exp2);
        }
        result
    }
}

impl<T, const WIDTH: usize> BitAnd<T> for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = Self;

    /// Bitwise-and matrix with an integer value.
    /// The meaning of this is, this modifies a matrix so that its matrix-multiplication with an
    /// integer represents a bit operation that has been masked with the given integer bit value.
    fn bitand(self, rhs: T) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.columns[i] & rhs;
        }
        result
    }
}

impl<T, const WIDTH: usize> Shl<usize> for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = Self;

    /// Shift a matrix left.
    /// The meaning of this is, this modifies a matrix so that its matrix-multiplication with an
    /// integer represents a bit operation that is shifted left.
    fn shl(self, n: usize) -> BCMatrix<T, WIDTH> {
        let mask = crate::math::bit_width_mask(WIDTH);
        let mut result = BCMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = (self.columns[i] << n) & mask;
        }
        result
    }
}

impl<T, const WIDTH: usize> Shr<usize> for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = Self;

    /// Shift a matrix right. The meaning of this is, this modifies a matrix so that its
    /// matrix-multiplication with an integer represents a bit operation that is shifted right.
    fn shr(self, n: usize) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.columns[i] >> n;
        }
        result
    }
}

impl<T, const WIDTH: usize> core::ops::Add for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = Self;

    /// Add two matrices.
    fn add(self, b: BCMatrix<T, WIDTH>) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }
}

impl<'a, 'b, T, const WIDTH: usize> core::ops::Add<&'b BCMatrix<T, WIDTH>>
    for &'a BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = BCMatrix<T, WIDTH>;

    /// Add two matrices (by reference).
    fn add(self, b: &'b BCMatrix<T, WIDTH>) -> BCMatrix<T, WIDTH> {
        let mut result = BCMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }
}

impl<T, const WIDTH: usize> core::ops::Mul for BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = Self;

    /// Multiply two matrices.
    fn mul(self, b: BCMatrix<T, WIDTH>) -> BCMatrix<T, WIDTH> {
        self.dot(&b)
    }
}

impl<'a, 'b, T, const WIDTH: usize> core::ops::Mul<&'b BCMatrix<T, WIDTH>>
    for &'a BCMatrix<T, WIDTH>
where
    T: BitMatrixInt,
{
    type Output = BCMatrix<T, WIDTH>;

    /// Multiply two matrices (by reference).
    fn mul(self, b: &'b BCMatrix<T, WIDTH>) -> BCMatrix<T, WIDTH> {
        self.dot(b)
    }
}
