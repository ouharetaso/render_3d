#![feature(generic_const_exprs)]

mod vector;
use vector::Vector;
mod matrix;
use matrix::Matrix;
//use crate::matrix::MatrixProduct;
//use crate::matrix::VectorProduct;

mod transform;

mod homogeneous;

mod framebuffer;
use framebuffer::FrameBuffer;

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
            println!( "| {:5} |   | {:5} | = | {:5} |", m[i], n[i], m.prod_mat(n)[i] );
        }
        else {
            println!( "| {:5} |   | {:5} |   | {:5} |", m[i], n[i], m.prod_mat(n)[i] );
        }
    }

    println!("{}", m.prod_vec(a));

    let mut image = FrameBuffer::new(100, 100);

    image.set_pixel(50, 50, Vector{ v : [255, 255, 255] });

    image.write_image("test.png");
}
