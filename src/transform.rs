use num_traits::real::Real;
use num_traits::Num;

use crate::Vector;
use crate::Matrix;


type HomogeneousVector<T, const N: usize> = Vector<T,{N+1}>;