trait Mul<RHS, Output> {
    fn mul(self, rhs: RHS) -> Output;
}

impl Mul<f64, f64> for f64 {
    fn mul(self, rhs: f64) -> f64 {
        self * rhs
    }
}

#[derive(Debug, Clone, Copy)]
struct Complex {
    x: f64,
    y: f64,
}

impl Complex {
    fn new(x: f64, y: f64) -> Complex {
        Complex { x: x, y: y }
    }
}

impl Mul<f64, Complex> for Complex {
    fn mul(self, rhs: f64) -> Complex {
        Complex::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<f64, f64> for Complex {
    fn mul(self, rhs: f64) -> f64 {
        self.x * rhs
    }
}

impl Mul<Complex, Complex> for Complex {
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(self.x * rhs.x - self.y * rhs.y, self.x * rhs.y + self.y * rhs.x)
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Complex {
        Complex::new(self.x * rhs, self.y * rhs)
    }
}

// Compiler error:
// impl std::ops::Mul<f64> for Complex {
//     type Output = f64;
//     fn mul(self, rhs: f64) -> Complex {
//         self.x * rhs
//     }
// }


fn main() {
    let x = 3.0.mul(4.2);

    let z1 = Complex::new(1.0, 2.3);
    let z2 = Complex::new(1.0, 2.0);

    let x2: f64 = z1.mul(x);
    println!("x2: {}", x2);
    
    let z3: Complex = z1.mul(x);
    println!("{:?}", z3);
    println!("{:?}", z1.mul(z2));
}
