// Non impl Trait approach:
// Define a closure type explicitly
type MyClosure = dyn Fn(i32) -> i32;

// Function that returns a closure
fn returns_closure_explicitly() -> Box<MyClosure> {
    Box::new(|x| x + 1)
}


// Use impl Trait type approach:
// Function that takes a closure as input and applies it to a value
fn apply_closure(closure: impl Fn(i32) -> i32, value: i32) -> i32 {
    closure(value)
}
fn returns_closure_impl_trait() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    /* closure syntax variants:
        fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32|          x + 1  ;
    */
    // => different syntaxes but same feature


    /* closure: */
    let plus_one = |x: i32| x + 1;
    println!("{}", plus_one(2));

    /* another closure example */
    // annotate the types of arguments the closure takes or the values it returns
    let plus_one = |x: i32| -> i32 { x + 1 };
    println!("{}", plus_one(7));

    /* multiple line closure */
    let plus_two = |x| {
        let mut result: i32 = x;
    
        result += 1;
        result += 1;
    
        result
    };
    println!("{}", plus_two(8));
    



    /* Non impl Trait: */
    // Call the returns_closure function and assign the closure to a variable
    let closure = returns_closure_explicitly();

    // Call the closure with a value
    let result = closure(5);
    println!("Result of the closure: {}", result);  // 6

    

    
    /* Use impl Trait type: */ 
    // Call the returns_closure function and assign the closure to a variable
    let closure = returns_closure_impl_trait();

    // Call the closure directly with a value
    let result = closure(5);
    println!("Result of the closure: {}", result);  // 6

    // Alternatively, you can use the apply_closure function
    let another_result = apply_closure(closure, 10);    // 11
    println!("Another result: {}", another_result);

}


#[test]
fn test_plus_one_closure() {
    // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/closures.html
    let plus_one = |x: i32| x + 1;
    assert_eq!(5, plus_one(4));
}

#[test]
fn test_plus_one_closure1() {
    // annotate the types of arguments the closure takes or the values it returns
    let plus_one = |x: i32| -> i32 { x + 1 };
    assert_eq!(2, plus_one(1));
}

#[test]
fn test_plus_two_closure() {
    let plus_two = |x| {
        let mut result: i32 = x;
    
        result += 1;
        result += 1;
    
        result
    };
    assert_eq!(10, plus_two(8));
}

#[test]
fn test_impl_trait_type_closure() {
    let closure = returns_closure_impl_trait();

    let result = closure(5);
    assert_eq!(6, result);
}