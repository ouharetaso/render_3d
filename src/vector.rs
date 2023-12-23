use std::ops::*;
use std::fmt;
use num_traits::real::Real;
use num_traits::cast::NumCast;


#[derive(Debug, Copy, Clone)]
pub struct Vector <T: Real, const N: usize> {
    pub v : [T; N],    
}


impl<T: Real, const N: usize> Default for Vector <T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Vector {
            v: [T::default(); N],
        }
    }
}


impl<T: Real, const N: usize> std::fmt::Display for Vector <T, N>
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = String::from("[");
        for i in 0..N {
            formatted.push_str(&self[i].to_string());
            if i < N-1 {
                formatted.push_str(", ");
            }
            else{
                ()
            }
        }
        formatted.push_str("]");
        write!(f, "{}", formatted)
    }
}


/*
    Operators for Vector

    Neg
    Add
    AddAssign
    Sub
    SubAssign
    Mul
    Index
    IndexMut
*/

impl <T: Real, const N: usize> Neg for Vector <T, N>
where 
    T: std::ops::Neg<Output = T> + Default
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = -self[i];
        }

        return ret;
    }
}


impl <T: Real, const N: usize> Add for Vector <T, N>
where 
    T: std::ops::Add<Output = T> + Default
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = self[i] + other[i];
        }

        return ret;
    }
}


impl <T: Real, const N: usize> AddAssign for Vector <T, N>
where 
    T: std::ops::AddAssign
{
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self[i] += other[i]
        }
    }
}


impl <T: Real, const N: usize> Sub for Vector <T, N>
where 
    T: std::ops::Sub<Output = T> + Default
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = self[i] - other[i];
        }

        return ret;
    }
}


impl <T: Real, const N: usize> SubAssign for Vector <T, N>
where 
    T: std::ops::SubAssign
{
    fn sub_assign(&mut self, other: Self) {
        for i in 0..N {
            self[i] -= other[i]
        }
    }
}


impl <T: Real, const N: usize, U: Real> Mul<U> for Vector <T, N>
where 
    T: std::ops::Mul<Output = T> + Default,
    U: Into<T>
{
    type Output = Self;

    fn mul(self, other: U) -> Self::Output {
        let mut ret: Vector<T, N> = Default::default();
        for i in 0..N {
            ret[i] = self[i] * Into::<T>::into(other);
        }

        return ret;
    }
}


macro_rules! defmulvec {
    ($($t: ty), *) => {
        $(
            impl<T: Real, const N: usize> Mul<Vector<T, N>> for $t
            where 
                T: std::ops::Mul<Output = T> + Default + NumCast
            {   
                type Output = Vector<T, N>;
                fn mul(self, other: Vector<T, N>) -> Self::Output{
                    let mut ret: Vector<T, N> = Default::default();
                    for i in 0..N {
                        let converted: Option<T> = NumCast::from(self);
                        match converted {
                            Some(value) => {
                                ret[i] = value * other[i];
                            }
                            None => (),
                        }
                    }
            
                    return ret;
                }
            }
        )*
    };
}


defmulvec![f32, f64, i8, i16, i32, i64, u8, u16, u32, u64];


impl<T: Real, const N: usize> Index<usize> for Vector <T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.v[index]
    }
}


impl<T: Real, const N: usize> IndexMut<usize> for Vector <T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.v[index]
    }
}


/*
    Operations that vector should have:

    dot
    cross
    len
*/
pub trait VectorOperation<T: Real, const N: usize>{
    fn dot(&self, other: &Self) -> T;
    fn cross(&self, other: &Self) -> Self;
    fn len(&self) -> T;
}


impl <T: Real, const N: usize> VectorOperation<T, N> for Vector<T, N>
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

    fn len(&self) -> T {
        let mut ret: T = T::default();
        for i in 0..N {
            ret += self[i] * self[i];
        }

        return ret.sqrt();
    }
}