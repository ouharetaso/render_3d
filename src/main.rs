mod vector;
use vector::Vector;


fn main() {
    let a : Vector<f32, 3> = Vector{ v : [1., 1., 4.] };
    let b : Vector<f32, 3> = Vector{ v : [5., 1., 4.] };
    
    println!("{} + {} = {}", a, b, a + b);
    println!("{} * {} = {}", 4, b, 4 * b);
    
}
