use num_traits::Num;

use crate::Vector;
use crate::Matrix;


pub type HomogeneousVector<T, const N: usize> = Vector<T, {N + 1}>;
pub type HomogeneousMatrix<T, const N: usize> = Matrix<T, {N + 1}>;


trait HomogenizeVector<T: Num, const N: usize>
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn homogenize(&self) -> HomogeneousVector<T, N>;
}


impl<T: Num, const N: usize> HomogenizeVector<T, N> for Vector<T, N> 
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn homogenize(&self) -> HomogeneousVector<T, N> {
        let mut ret: HomogeneousVector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = self[i];
        }
        ret[N] = T::one();

        return ret;
    }
}


trait DehomogenizeVector<T: Num, const N: usize> {
    fn dehomogenize(&self) -> Vector<T, N>;
}


impl<T: Num, const N: usize> DehomogenizeVector<T, N> for HomogeneousVector<T, N> 
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn dehomogenize(&self) -> Vector<T, N> {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = self[i] / self[N];
        }

        return ret;
    }
}


trait HomogenizeMatrix<T: Num, const N: usize>
where   
    [(); N + 1]: ,
    T: Default + Copy
{
    fn homogenize(&self) -> HomogeneousMatrix<T, N>;
}


trait DehomogenizeMatrix<T: Num, const N: usize> {
    fn dehomogenize(&self) -> Matrix<T, N>;
}


impl<T: Num, const N: usize> HomogenizeMatrix<T, N> for Matrix<T, N> 
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn homogenize(&self) -> HomogeneousMatrix<T, N> {
        let mut ret: HomogeneousMatrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i][j] = self[i][j];
            }
            ret[i][N] = T::zero();
        }
        for j in 0..N {
            ret[N][j] = T::zero();
        }
        ret[N][N] = T::one();

        return ret;
    }
}


impl <T: Num, const N: usize> DehomogenizeMatrix<T, N> for HomogeneousMatrix<T, N> 
where
    [(); N + 1]: ,
    T: Default + Copy
{
    fn dehomogenize(&self) -> Matrix<T, N> {
        let mut ret: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                ret[i][j] = self[i][j] / self[N][N];
            }
        }

        return ret;
    }
}