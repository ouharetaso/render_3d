use num_traits::Num;
use num_traits::Float;

use crate::Vector;
use crate::Matrix;

//use crate::matrix::MatrixProduct;
//use crate::matrix::VectorProduct;

use crate::homogeneous::HomogeneousVector;
use crate::homogeneous::HomogeneousMatrix;


pub trait Transform<T: Num + Float, const N: usize>
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn translate(&self, v: &Vector<T, N>) -> Self;
    fn scale(&self, v: Vector<T, N>) -> Self;
    fn rotate(&self, theta: T, idx: usize) -> Self;
}


impl<T: Num + Float, const N: usize> Transform<T, N> for HomogeneousMatrix<T, N>
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn translate(&self, v: &Vector<T, N>) -> Self {
        let mut t: HomogeneousMatrix<T, N> = HomogeneousMatrix::<T, N>::identity();
        for i in 0..N {
            t[i][N] = v[i];
        }

        return t;
    }

    fn scale(&self, v: Vector<T, N>) -> Self {
        let mut s: HomogeneousMatrix<T, N> = HomogeneousMatrix::<T, N>::identity();
        for i in 0..N {
            s[i][i] = v[i];
        }
        return s;
    }

    fn rotate(&self, theta: T, idx: usize) -> Self{
        let mut r: HomogeneousMatrix<T, N> = HomogeneousMatrix::<T, N>::identity();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        r[idx][idx] = cos_theta;
        r[idx][(idx + 1) % N] = -sin_theta;
        r[(idx + 1) % N][idx] = sin_theta;
        r[(idx + 1) % N][(idx + 1) % N] = cos_theta;

        return r;
    }
}


impl<T: Num + Float, const N: usize> Transform<T, N> for HomogeneousVector<T, N>
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn translate(&self, v: &Vector<T, N>) -> Self {
        let mut t: HomogeneousMatrix<T, N> = HomogeneousMatrix::<T, N>::identity();
        for i in 0..N {
            t[i][N] = v[i];
        }

        return t.prod_vec(*self);
    }

    fn scale(&self, v: Vector<T, N>) -> Self {
        let mut s: HomogeneousMatrix<T, N> = HomogeneousMatrix::<T, N>::identity();
        for i in 0..N {
            s[i][i] = v[i];
        }
        return s.prod_vec(*self);
    }

    fn rotate(&self, theta: T, idx: usize) -> Self{
        let mut r: HomogeneousMatrix<T, N> = HomogeneousMatrix::<T, N>::identity();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        r[idx][idx] = cos_theta;
        r[idx][(idx + 1) % N] = -sin_theta;
        r[(idx + 1) % N][idx] = sin_theta;
        r[(idx + 1) % N][(idx + 1) % N] = cos_theta;

        return r.prod_vec(*self);
    }
}
