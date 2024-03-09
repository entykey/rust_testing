

// fn main() {
//     let mut scores = vec![1, 2, 3];
//     let first_elm_of_scores = &scores[0];
//     scores.push(4);
    
//     println!("{}", first_elm_of_scores);
// }

// the above code won't compile and will produce the following error:
/*
let first_elm_of_scores = &scores[0];
  |                                ------ immutable borrow occurs here
6 |     scores.push(4);
  |     ^^^^^^^^^^^^^^ mutable borrow occurs here
7 |     
8 |     println!("{}", first_elm_of_scores);
  |                    ------------------- immutable borrow later used here
*/


// fix
fn main() {
    let mut scores = vec![1, 2, 3];
    let score = &scores[0];
    scores.push(4);
    
    let first_elm_of_scores = &scores[0]; // Reassign score
    println!("{}", first_elm_of_scores);
}
