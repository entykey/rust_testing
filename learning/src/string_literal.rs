fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {

    let mut my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    println!("word: {}", word);

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("word: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word: {}", word);

    println!("my_string: {}", my_string);   // my_string still valid here

    my_string.clear(); // this empties the String, making it equal to ""
    println!("my_string: {}", my_string);   // value: ""



    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..=3];
    println!("slice: {:?}", slice);

}
