// Static Dispatch saves more performance than Dynamic Dispatch
// https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}


// perform static dispatch with trait bounds
// fn do_something<T: Foo>(x: T) {
//     println!("do_something called: {}", x.method());
//     x.method();      // => moved forever (after free), other later function can't borrow
// }

// use Move & Return
// fn do_something<T: Foo>(x: T) -> T {
//     println!("do_something called: {}", x.method());
//     x.method();
//     x // transfer ownership of the value back to the caller by returning it from the function
// }

// use References (Shared borrow: &T)
fn do_something<T: Foo>(x: &T) {
    println!("do_something called: {}", x.method());
    x.method();
}


// use 'monomorphization' to perform static dispatch with less performance overhead
fn do_something_u8(x: &u8) {
    println!("do_something_u8 called: {}", x.method());
    x.method();
}

fn do_something_string(x: &String) {
    println!("do_something_string called {}", x.method());
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    println!("\n> generic with trait bounds: ");
    do_something(&x);
    do_something(&y);

    println!("\n> monomorphization: ");
    do_something_u8(&x);
    do_something_string(&y);
}