use std::ops::*;
use num_traits::real::Real;

use crate::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Matrix <T: Real, const N: usize> {
    pub m : [Vector<T, N>; N]
}


impl<T: Real, const N: usize> Default for Matrix <T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Matrix {
            m: [Default::default(); N]
        }
    }
}


impl<T: Real, const N: usize> Index<usize> for Matrix <T, N> {
    type Output = Vector<T, N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}


impl<T: Real, const N: usize> IndexMut<usize> for Matrix <T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.m[index]
    }
}


pub trait MatrixProduct<T: Real, const N: usize> {
    fn product(&self, other: Matrix<T, N>) -> Self;
}


impl<T: Real, const N: usize> MatrixProduct<T, N> for Matrix<T, N>
where
    T: Default + std::ops::AddAssign
{
    fn product(&self, other: Matrix<T, N>) -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    ret[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        return ret;
    }
}


impl<T: Real, const N: usize> MatrixProduct<T, N> for Vector<T, N>
where
    T: Default + std::ops::AddAssign
{
    fn product(&self, other: Matrix<T, N>) -> Self {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i] += self[j] * other[j][i];
            }
        }
        return ret;
    }
}


pub trait VectorProduct<T: Real, const N: usize> {
    fn product(&self, other: Vector<T, N>) -> Vector<T, N>;
}


impl<T: Real, const N: usize> VectorProduct<T, N> for Matrix<T, N>
where
    T: Default + std::ops::AddAssign
{
    fn product(&self, other: Vector<T, N>) -> Vector<T, N> {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i] += self[i][j] * other[j];
            }
        }
        return ret;
    }
}
