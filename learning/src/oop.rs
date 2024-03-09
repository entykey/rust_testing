use std::fmt;


trait Shape: fmt::Display {
    fn area(&self) -> f64;

    // fn new(width: f64, height: f64) -> Self;
    fn new(params: Vec<f64>) -> Self;
}

struct Rectangle {
    width: f64,
    height: f64,
}
struct Circle {
    radius: f64,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle (width: {}, height: {})", self.width, self.height)
    }
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle with radius {:.2}", self.radius)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // fn new(width: f64, height: f64) -> Self {
    //     Rectangle { width, height }
    // }
    fn new(params: Vec<f64>) -> Self {
        let width = params[0];
        let height = params[1];
        Rectangle { width, height }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn new(params: Vec<f64>) -> Self {
        let radius = params[0];
        Circle { radius }
    }
}

fn main() {
    let start = std::time::Instant::now();

    let rectangle_params = vec![5.76, 10.into()];
    let circle_params = vec![3.14];

    let rectangle: Rectangle = Rectangle::new(rectangle_params);
    println!("{}", rectangle);
    println!("Area: {}", rectangle.area());

    println!("");

    let circle: Circle = Circle::new(circle_params);
    println!("{}", circle);
    println!("Area: {}", circle.area());

    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
