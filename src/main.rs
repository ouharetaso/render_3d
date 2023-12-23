mod vector;
use vector::Vector;
mod matrix;
use matrix::Matrix;
use crate::matrix::MatrixProduct;

fn main() {
    let a : Vector<f32, 3> = Vector{ v : [1., 1., 4.] };
    let b : Vector<f32, 3> = Vector{ v : [5., 1., 4.] };
    let m : Matrix<f32, 3> = Matrix{ m : [
        Vector{ v : [1., 1., 4.] },
        Vector{ v : [5., 1., 4.] },
        Vector{ v : [8., 1., 0.] }    
    ]};
    let n : Matrix<f32, 3> = Matrix{ m : [
        Vector{ v : [0., 0., 0.] },
        Vector{ v : [8., 9., 3.] },
        Vector{ v : [-1.25, -0.75, 0.25] }    
    ]};
    
    println!("{} + {} = {}", a, b, a + b);
    println!("{} * {} = {}", 4, b, 4 * b);
    for i in 0..3 {
        if i == 1 {
            println!( "| {} |   | {} | = | {} |", m[i], n[i], m.product(n)[i] );
        }
        else {
            println!( "| {} |   | {} |   | {} |", m[i], n[i], m.product(n)[i] );
        }
    }
}
