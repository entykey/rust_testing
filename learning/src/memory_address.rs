fn main() {
    let my_vec = vec![1, 2, 3];

    let my_vec_address = std::ptr::addr_of!(my_vec);
    println!("ptr of my_vec: {:?}", my_vec_address);

    let x = 10;
    let ptr_of_x = &x as *const i32;
    println!("ptr of : {:?}", ptr_of_x);

    
    println!();
    let vec = vec![1, 2, 13, 4, 5];
    let element = &vec[2];
    let ptr = element as *const i32;
    
    println!("Memory address of vec[2]: {:p}", ptr);
    // Rust's memory safety rules prevent you from dereferencing raw pointers without proper safety measures
    // However, if you have a valid reference or pointer and you want to access the value it points to,
    // you can do so using the unsafe keyword and the ptr::read function from the std::ptr module
    unsafe {
        let value = std::ptr::read(ptr);
        println!("Value at memory address {:p}: {}", ptr, value);
    }


    println!();
    // get a raw pointer points to the data
    let my_vec_ptr = my_vec.as_ptr();
    // convert the raw pointer to an integer
    let my_vec_addr = my_vec_ptr as usize;

    println!("address = {:X}", my_vec_addr);
}