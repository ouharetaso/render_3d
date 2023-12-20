use std::ops::*;

pub struct Vector <T, const N: usize> {
    pub v : [T; N],    
}


impl<T, const N: usize> Default for Vector <T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Vector {
            v: [T::default(); N],
        }
    }
}


impl<T, const N: usize> Index<usize> for Vector <T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.v[index]
    }
}


impl<T, const N: usize> IndexMut<usize> for Vector <T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.v[index]
    }
}


pub trait VectorOperation<T, const N: usize>{
    fn dot(&self, other: &Self) -> T;
    fn cross(&self, other: &Self) -> Self;
}


impl <T, const N: usize> VectorOperation<T, N> for Vector<T, N>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>
     + std::ops::AddAssign + Default + Copy, 
{
    fn dot(&self, other: &Self) -> T {
        let mut ret: T = T::default();
        for i in 0..N {
            ret += self[i] * other[i];
        }

        return ret;
    }

    fn cross(&self, other: &Self) -> Self {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = self[(i + 1) % N] * other[(i + 2) % N] - self[(i + 2) % N] * other[(i + 1) % N];
        }
        return ret;
    }

}