use num::Complex;
use numbrs::Matrix;
fn main() {
    let x1 = 2.0_f64.sqrt();
    let x2 = 3.0_f64.sqrt();

    println!("{:?}", x1);
    println!("{:?}", x2);

    let v = Matrix::new([[x1, x2]]);

    println!("{:?}", v);

    // let c = complex!(1+2 i);

    // let c = complex!(1+2i);
    let c = Complex::new(1.0, 2.0);
    println!("{:?}", c);

    println!("{:?}", c.to_polar());
}
