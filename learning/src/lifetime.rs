/* Made it work: https://practice.rs/lifetime/static.html */
use std::fmt::Debug;

// won't compile !
// fn print_it<T: Debug + 'static>( input: T) {
//     println!( "'static value passed in is: {:?}", input );
// }


fn print_it<'a, T: Debug>(input: &'a T) {
    println!("'{}' value passed in is: {:?}", std::any::type_name::<T>(), input);
}

// won't compile !
// fn print_it1( input: impl Debug + 'static ) {
//     println!( "'static value passed in is: {:?}", input );
// }
fn print_it1<'a>(input: &'a impl Debug) {
    println!("'{}' value passed in is: {:?}", std::any::type_name::<&dyn Debug>(), input);
}

fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // or just use `const i: i32 = 5;` and it will have 'static lifetime
    let i = 5;
    let ch: char = '👌';
    let str: [char; 2] = ['👌', '👌'];
    let bx: Box<i32> = Box::new(3.8 as i32);
    //print_it(i);  // won't compile

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
    print_it(&ch);
    print_it(&str);
    print_it(&bx);

    print_it1(&i);
    print_it1(&ch);
    print_it1(&str);
    

    // but this one WORKS !
    print_it2(&i);
}
