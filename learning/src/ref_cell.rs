// In this example, we define a Counter struct that contains a RefCell<i32>. 
// The increment method mutates the count even though self is immutable because it 
// borrows a mutable reference to the RefCell. Similarly, the get_count method borrows
// an immutable reference to the RefCell to read the count, allowing us to access
// the count even though self is immutable. This demonstrates how RefCell provides interior
// mutability for a single value with shared references, allowing us to bypass Rust's
// borrowing rules at runtime. However, we need to be careful when using RefCell to avoid
// panics caused by borrowing violations.
use std::cell::RefCell;

struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: RefCell::new(0),
        }
    }

    fn increment(&self) {
        // We can mutate the count even though `self` is immutable,
        // because we are using a `RefCell` to provide interior mutability.
        *self.count.borrow_mut() += 1;
    }

    fn get_count(&self) -> i32 {
        // We can get a shared reference to the count even though `self` is immutable,
        // because we are using a `RefCell` to enable interior mutability with shared references.
        *self.count.borrow()
    }
}

fn main() {
    let counter = Counter::new();

    // We can call `increment` on an immutable reference to the `Counter`,
    // because `increment` uses `RefCell` to provide interior mutability.
    for _ in 0..10 {
        counter.increment();
    }

    // We can also get the count using a shared reference,
    // because `get_count` uses `RefCell` to enable interior mutability with shared references.
    println!("Count: {}", counter.get_count());
}
