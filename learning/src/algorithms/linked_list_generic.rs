/*use std::ptr;

#[derive(Debug, PartialEq)]
pub struct Node {
    data: f32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    pub fn new(data: f32) -> *mut Self {
        Box::into_raw(Box::new(Node {
            data,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }

    pub fn display(&self) {
        print!("{} ", self.data);
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList {
    head: *mut Node,
    tail: *mut Node,
    size: i32,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn display(&self) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                (*current).display();
                current = (*current).next;
            }
        }
    }

    pub fn add_head(&mut self, data: f32) {
        let new_node = Node::new(data);
        unsafe {
            if self.head.is_null() {
                self.head = new_node;
                self.tail = new_node;
            } else {
                (*new_node).next = self.head;
                (*self.head).prev = new_node;
                self.head = new_node;
            }
            self.size += 1;
        }
    }

    pub fn add_tail(&mut self, data: f32) {
        let new_node = Node::new(data);
        unsafe {
            if self.tail.is_null() {
                self.head = new_node;
                self.tail = new_node;
            } else {
                (*new_node).prev = self.tail;
                (*self.tail).next = new_node;
                self.tail = new_node;
            }
            self.size += 1;
        }
    }

    // Add other methods similar to the C++ code

    // ...

    pub fn clear(&mut self) {
        while !self.head.is_null() {
            let temp = self.head;
            unsafe {
                self.head = (*temp).next;
                //Box::from_raw(temp);
            }
        }
        self.tail = ptr::null_mut();
        self.size = 0;
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.add_tail(1.0);
    list.add_tail(2.0);
    list.add_tail(3.0);

    println!("Doubly linked list:");
    list.display();

    // Add more operations similar to the C++ code

    // ...
}
*/

// https://rust-unofficial.github.io/too-many-lists/second-generic.html
#[derive(Debug, PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

// impl<T: PartialEq> List<T> {
//     pub fn search(&self, target: &T) -> Option<&T> {
//         let mut current_link = &self.head;

//         while let Some(ref boxed_node) = current_link {
//             if &boxed_node.elem == target {
//                 return Some(&boxed_node.elem);
//             }
//             current_link = &boxed_node.next;
//         }

//         None
//     }
// }


// implement search by value, return element's value & address
impl<T: PartialEq> List<T> {
    pub fn search(&self, target: &T) -> Option<(&T, *const T)> {
        let mut current_link = &self.head;

        while let Some(ref boxed_node) = current_link {
            if &boxed_node.elem == target {
                let element_ref: *const T = &boxed_node.elem;
                return Some((&boxed_node.elem, element_ref));
            }
            current_link = &boxed_node.next;
        }

        None
    }
}

// display(show) entire list with Display trait
impl<T: std::fmt::Display> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut current_link = &self.head;
        write!(f, "List: [")?;

        while let Some(ref boxed_node) = current_link {
            write!(f, "{}", boxed_node.elem)?;

            current_link = &boxed_node.next;
            if current_link.is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

fn main() {
    let main_time = std::time::Instant::now();
    
    let mut list1 = List::<&str>::new();
    list1.push("hello");
    list1.push("world!");

    println!("{:?}", list1);
    println!("{}", list1);

    // search
    if let Some(result) = list1.search(&"world!") {
        println!("Found: {:?}", result);
    } else {
        println!("Element not found");
    }

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_seconds = elapsed_ms / 1000.0; // Convert milliseconds to seconds
    println!("\nExecution time: {:?} ({:?} ms) ({:.8} s)", duration, elapsed_ms, elapsed_seconds);
}