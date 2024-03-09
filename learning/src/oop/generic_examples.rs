use std::fmt::{Debug, Display, Formatter, Result};


// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
#[derive(Debug)]
struct Point<'a, T: 'a, U: 'a> {
    x: &'a T,
    y: &'a U,
}

// add a trait bound to the generic types T and U in the Point struc
// fn print_point<T: Debug, U: Debug>(point: &Point<T, U>) {
//     println!("x: {:?}, y: {:?}", point.x, point.y);
// }


// use `where` clause to apply the trait bound to both types simultaneously
// fn print_point<T, U>(point: &Point<T, U>)
// where
//     T: Debug,
//     U: Debug,
// {
//     println!("x: {:?}, y: {:?}", point.x, point.y);
// }


// fn print_point<'a, T, U>(point: &'a Point<T, U>)
// where
//     T: Debug,
//     U: Debug,
// {
//     println!("x: {:?}, y: {:?}", point.x, point.y);
//     // println!("{}", self);
// }


impl<'a, T:Debug, U:Debug> Point<'a, T, U> {
    fn generic_print_point(&self) {
        print!("{}", self)
    }
}

// implement fmt::Display for a struct with a generic type
impl<'a, T:Debug, U:Debug> Display for Point<'a, T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "x: {:?}, y: {:?}", self.x, self.y) // T doesn't implement `Debug`
    }
}




trait Shape: Send + Sync {
    fn area(&self) -> f64;
    fn introduce(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn introduce (&self) {
        // println!("This is a Circle with radius = {}", self.radius);
        println!("{}", self);
    }
}

// Implement the Default trait for Circle
impl Default for Circle {
    fn default() -> Self {
        Circle { radius: 0.0 } // Some default value, change it to whatever makes sense
    }
}

// Implement the fmt::Display trait for Circle
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "[fmt::Display] This is a Circle with radius = {}", self.radius)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn introduce (&self) {
        // println!("This is a Square with side = {}", self.side);
        print!("{}", self);
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "[fmt::Display] This is a Square with side = {}", self.side)
    }
}


// polymorphism approach
// The usage of dyn here enables dynamic dispatch,
// so the appropriate area() method will be called at runtime based on the actual type of the object.
// fn print_area(shape: &dyn Shape) {
//     println!("Area: {}", shape.area());
// }

// generic approach, with Send & Sync traits for thread safety
// fn print_area<T: Shape + Send + Sync>(shape: &T) {
//     println!("Area: {}", shape.area());
// }
// fn generic_introduce<T: Shape + Send + Sync>(shape: &T) {
//     shape.introduce();
// }
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}
fn generic_introduce(shape: &dyn Shape) {
    shape.introduce();
}
fn dynamic_dispatch_introduce(shape: &dyn Shape) {
    shape.introduce();
}


// lifetime example:

// fn find_longest(s1: &str, s2: &str) -> &'static str {
//     if s1.len() >= s2.len() {
//         s1  // -> lifetime may not live long enough, returning this value requires that `'2` must outlive `'static`
//     } else {
//         s2  // -> lifetime may not live long enough, returning this value requires that `'2` must outlive `'static`
//     }
// }

// The 'a lifetime specifies that the function will return a reference to a slice that lives
// at least as long as the lifetime 'a.
fn find_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

// https://doc.rust-lang.org/book/ch10-01-syntax.html   (find longer string or larger number)
fn largest_num_or_longest_str<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


#[allow(dead_code)]
fn loop_iter<V>(v: V)
where for<'a> &'a V: IntoIterator<Item=&'a i32>,
      V: IntoIterator<Item=i32>
{
    let itr1 = (&v).into_iter();
    for xi in itr1 {
        println!("{}", xi);
    }
    println!();
    for zi in v {
        println!("{}", zi);
    }
}



fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    
    // without lifetiime
    // let float = Point { x: 4.0, y: 1.0 };

    // let mixed = Point { x: 2, y: 4.8 };

    // println!("{:?}", float);
    // println!("{:?}", mixed);

    // print_point(&float);
    // print_point(&mixed);


    // with lifetime
    let x = 4.0;
    let y = 1.0;
    let float = Point { x: &x, y: &y };

    let x = 2;
    let y = 4.8;
    let mixed = Point { x: &x, y: &y };

    println!("{:?}", float);
    println!("{:?}", mixed);

    // print_point(&float);
    // print_point(&mixed);
    float.generic_print_point();
    mixed.generic_print_point();



    println!("\n###   Example with shapes     ###\n");
    //let circle = Circle { radius: 5.0 };
    let circle = Circle::default();
    let square = Square { side: 4.0 };

    // generic & dynamic dispatch way:
    generic_introduce(&circle);
    print_area(&circle);

    // normal way:
    println!("Normal Area: {}", circle.area());

    generic_introduce(&square);
    print_area(&square);


    println!("\n###  dynamic_dispatch_introduce   ###\n");
    dynamic_dispatch_introduce(&circle);
    dynamic_dispatch_introduce(&square);



    println!("\n###   Send & Sync traits      ###\n");
    let default_circle = Circle::default();
    generic_introduce(&default_circle);

    let shape_circle: Box<dyn Shape + Send + Sync> = Box::new(Circle { radius: 2.0 });
    generic_introduce(&*shape_circle);
    print_area(&*shape_circle);

    let shape_square: Box<dyn Shape + Send + Sync> = Box::new(Square { side: 3.0 });
    generic_introduce(&*shape_square);
    print_area(&*shape_square);
    



    println!();
    // lifetime example
    let s1 = "hello";
    let s2 = "world";
    let longest = find_longest(s1, s2);
    println!("The longest string is: {}", longest);


    println!();
    // larger example:
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_num_or_longest_str(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_num_or_longest_str(&char_list);
    println!("The largest char is {}", result);


    // loop_iter(vec![1,2,3]);
    // println!();
    // loop_iter([1,5,9]);


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}