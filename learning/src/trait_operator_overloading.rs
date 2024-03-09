// https://doc.rust-lang.org/rust-by-example/trait/ops.html
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}




// Point adding example:  https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug, PartialEq)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> ops::Add<Point<T, U>> for Point<T, U>
where
    T: ops::Add<T>,
    U: ops::Add<U>,
    T::Output: Copy,  // New requirement
    U::Output: Copy,  // New requirement
{
    type Output = Point<T::Output, U::Output>;

    fn add(self, other: Point<T, U>) -> Point<T::Output, U::Output> {
        Point {
            x: self.x + other.x,    // T + T
            y: self.y + other.y,    // U + U
        }
    }
}



// since std::fmt::Display does not have a default implement like Debug, so we have to implement it manually
impl<T: std::fmt::Display, U: std::fmt::Display> std::fmt::Display for Point<T,U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> OutlinePrint for Point<T,U> {}

fn main() {
    let start = std::time::Instant::now();


    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);



    // point add example:
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 });
    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
    println!("{}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });

    // or print using outline_print()
    let point1 = Point { x: 1, y: 3.2 } + Point { x: 2, y: 3.0 };
    point1.outline_print();


    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}