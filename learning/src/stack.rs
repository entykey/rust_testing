// struct Stack<T> {
//     top: *mut Node<T>,
// }

// struct Node<T> {
//     value: T,
//     next: *mut Node<T>,
// }

// impl<T> Stack<T> {
//     fn new() -> Self {
//         Stack { top: std::ptr::null_mut() }
//     }

//     fn push(&mut self, value: T) {
//         let new_node = Box::new(Node {
//             value,
//             next: self.top,
//         });

//         self.top = Box::into_raw(new_node);
//     }

//     fn pop(&mut self) -> Option<T> {
//         if self.is_empty() {
//             return None;
//         }

//         unsafe {
//             let old_top = self.top;
//             self.top = (*old_top).next;
//             Some(Box::from_raw(old_top).value)
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.top.is_null()
//     }

//     fn iter(&self) -> StackIterator<T> {
//         StackIterator {
//             current: self.top,
//         }
//     }
// }

// struct StackIterator<T> {
//     current: *mut Node<T>,
// }

// impl<T> Iterator for StackIterator<T> {
//     type Item = (*mut Node<T>, T);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current.is_null() {
//             None
//         } else {
//             unsafe {
//                 let value = (*self.current).value;
//                 let address = self.current;
//                 self.current = (*self.current).next;
//                 Some((address, value))
//             }
//         }
//     }
// }

// fn main() {
//     let mut stack = Stack::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     for (address, value) in stack.iter() {
//         println!("Address: {:p}, Value: {}", address, value);
//     }
// }






// struct Stack<T> {
//     top: *mut Node<T>,
// }

// struct Node<T> {
//     value: T,
//     next: *mut Node<T>,
// }

// impl<T> Stack<T> {
//     fn new() -> Self {
//         Stack { top: std::ptr::null_mut() }
//     }

//     fn push(&mut self, value: T) {
//         let new_node = Box::new(Node {
//             value,
//             next: self.top,
//         });

//         self.top = Box::into_raw(new_node);
//     }

//     fn pop(&mut self) -> Option<T> {
//         if self.is_empty() {
//             return None;
//         }

//         unsafe {
//             let old_top = self.top;
//             self.top = (*old_top).next;
//             Some(Box::from_raw(old_top).value)
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.top.is_null()
//     }

//     fn iter(&self) -> StackIterator<T> {
//         StackIterator {
//             current: self.top,
//         }
//     }
// }

// struct StackIterator<T> {
//     current: *mut Node<T>,
// }

// impl<T> Iterator for StackIterator<T> {
//     type Item = (*mut Node<T>, T);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current.is_null() {
//             None
//         } else {
//             unsafe {
//                 let value = (*self.current).value.clone(); // Clone the value
//                 let address = self.current;
//                 self.current = (*self.current).next;
//                 Some((address, value))
//             }
//         }
//     }
// }

// fn main() {
//     let mut stack = Stack::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     for (address, value) in stack.iter() {
//         println!("Address: {:p}, Value: {}", address, value);
//     }
// }



// use std::fmt;

// struct Stack<T> {
//     top: *mut Node<T>,
// }

// struct Node<T> {
//     value: T,
//     next: *mut Node<T>,
// }

// impl<T> Stack<T> {
//     fn new() -> Self {
//         Stack { top: std::ptr::null_mut() }
//     }

//     fn push(&mut self, value: T) {
//         let new_node = Box::new(Node {
//             value,
//             next: self.top,
//         });

//         self.top = Box::into_raw(new_node);
//     }

//     fn pop(&mut self) -> Option<T> {
//         if self.is_empty() {
//             return None;
//         }

//         unsafe {
//             let old_top = self.top;
//             self.top = (*old_top).next;
//             Some(Box::from_raw(old_top).value)
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.top.is_null()
//     }

//     fn iter(&self) -> StackIterator<T> {
//         StackIterator {
//             current: self.top,
//         }
//     }
// }

// struct StackIterator<T> {
//     current: *mut Node<T>,
// }

// impl<T> Iterator for StackIterator<T> {
//     type Item = (*mut Node<T>, &'static T);     // <-- lifetime error: the parameter type `T` may not live long enough
//     //...so that the reference type `&'static T` does not outlive the data it points 

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current.is_null() {
//             None
//         } else {
//             unsafe {
//                 let value_ref: &'static T = &(*self.current).value;
//                 let address = self.current;
//                 self.current = (*self.current).next;
//                 Some((address, value_ref))
//             }
//         }
//     }
// }

// impl<T> fmt::Pointer for Stack<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:p}", self)
//     }
// }

// fn main() {
//     let mut stack = Stack::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     for (address, value) in stack.iter() {
//         println!("Stack Address: {:p}, Value: {}", &stack, value);
//         println!("Node Address: {:p}, Value: {}", address, value);
//     }
// }


// 1 Err code:
// use std::fmt;

// struct Stack<T> {
//     top: *mut Node<T>,
// }

// struct Node<T> {
//     value: T,
//     next: *mut Node<T>,
// }

// impl<T> Stack<T> {
//     fn new() -> Self {
//         Stack { top: std::ptr::null_mut() }
//     }

//     fn push(&mut self, value: T) {
//         let new_node = Box::new(Node {
//             value,
//             next: self.top,
//         });

//         self.top = Box::into_raw(new_node);
//     }

//     fn pop(&mut self) -> Option<T> {
//         if self.is_empty() {
//             return None;
//         }

//         unsafe {
//             let old_top = self.top;
//             self.top = (*old_top).next;
//             Some(Box::from_raw(old_top).value)
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.top.is_null()
//     }

//     fn iter(&self) -> StackIterator<T> {
//         StackIterator {
//             current: self.top,
//         }
//     }
// }

// struct StackIterator<T> {
//     current: *mut Node<T>,
// }

// impl<'a, T: 'a> Iterator for StackIterator<T> { //<-- Err: the 
// // parameter `'a` is not constrained by the impl trait, self type, or predicates
// // unconstrained lifetime parameter
//     type Item = (*mut Node<T>, &'a T);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current.is_null() {
//             None
//         } else {
//             unsafe {
//                 let value_ref: &'a T = &(*self.current).value;
//                 let address = self.current;
//                 self.current = (*self.current).next;
//                 Some((address, value_ref))
//             }
//         }
//     }
// }

// impl<T> fmt::Pointer for Stack<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:p}", self)
//     }
// }

// fn main() {
//     let mut stack = Stack::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     for (address, value) in stack.iter() {
//         println!("Stack Address: {:p}, Value: {}", &stack, value);
//         println!("Node Address: {:p}, Value: {}", address, value);
//     }
// }



// Fixed code:
use std::fmt;

struct Stack<T> {
    top: *mut Node<T>,
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { top: std::ptr::null_mut() }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.top,
        });

        self.top = Box::into_raw(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let old_top = self.top;
            self.top = (*old_top).next;
            Some(Box::from_raw(old_top).value)
        }
    }

    fn is_empty(&self) -> bool {
        self.top.is_null()
    }

    fn iter<'a>(&'a self) -> StackIterator<'a, T> {
        StackIterator {
            current: self.top,
            marker: std::marker::PhantomData,
        }
    }
}

struct StackIterator<'a, T> {
    current: *mut Node<T>,
    marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for StackIterator<'a, T> {
    type Item = (*mut Node<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let value_ref: &'a T = &(*self.current).value;
                let address = self.current;
                self.current = (*self.current).next;
                Some((address, value_ref))
            }
        }
    }
}

impl<T> fmt::Pointer for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:p}", self)
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    for (address, value) in stack.iter() {
        println!("Stack Address: {:p}, Value: {}", &stack, value);
        println!("Node Address: {:p}, Value: {}", address, value);
    }
}
