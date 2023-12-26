use std::ops::*;
use num_traits::Num;

use crate::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Matrix <T: Num, const N: usize> {
    pub m : [Vector<T, N>; N]
}


trait MatTrait{}
impl<T: Num, const N: usize> MatTrait for Matrix<T, N>{}


impl<T: Num, const N: usize> Default for Matrix <T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Matrix {
            m: [Default::default(); N]
        }
    }
}


impl<T: Num, const N: usize> Index<usize> for Matrix <T, N> {
    type Output = Vector<T, N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}


impl<T: Num, const N: usize> IndexMut<usize> for Matrix <T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.m[index]
    }
}


impl<T: Num, const N: usize> Matrix<T, N>
where
    T: Default + Copy,
{
    pub fn transpose(&self) -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i][j] = self[j][i];
            }
        }
        return ret;
    }

    pub fn identity() -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            ret[i][i] = T::one();
        }
        return ret;
    }
}


impl<T: Num, const N: usize> Matrix<T, N>
where
    T: Default + Copy
{
    pub fn prod_vec(&self, v: Vector<T, N>) -> Vector<T, N> {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i] = ret[i] + self[i][j] * v[j];
            }
        }
        return ret;
    }

    pub fn prod_mat(&self, m: Matrix<T, N>) -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    ret[i][j] = ret[i][j] + self[i][k] * m[k][j];
                }
            }
        }
        return ret;
    }
}

/*
pub trait MatrixProduct<T: Num, U: VecTrait, const N: usize> {
    fn product(&self, other: U) -> Self;
}


impl<T: Num, U: VecTrait, const N: usize> MatrixProduct<T, U, N> for Matrix<T, N>
where
    T: Default + Copy
{
    fn product(&self, other: U) -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    ret[i][j] = ret[i][j] + self[i][k] * other[k][j];
                }
            }
        }
        return ret;
    }
}



pub trait MatrixProduct<T: Num, const N: usize> {
    fn product(&self, other: Matrix<T, N>) -> Self;
}


impl<T: Num, const N: usize> MatrixProduct<T, N> for Matrix<T, N>
where
    T: Default + Copy
{
    fn product(&self, other: Matrix<T, N>) -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    ret[i][j] = ret[i][j] + self[i][k] * other[k][j];
                }
            }
        }
        return ret;
    }
}


pub trait VectorProduct<T: Num, const N: usize> {
    fn product(&self, other: Vector<T, N>) -> Vector<T, N>;
}


impl<T: Num, const N: usize> VectorProduct<T, N> for Matrix<T, N>
where
    T: Default + Copy
{
    fn product(&self, other: Vector<T, N>) -> Vector<T, N> {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i] = ret[i] + self[i][j] * other[j];
            }
        }
        return ret;
    }
}
*/