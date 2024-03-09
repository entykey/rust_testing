// upcoder Mail Sinh Vien
#![allow(unused)]
use std::collections::HashMap;
use std::io;    // for taking user's input from terminal

fn format_name(name: &str) -> String {
    let mut formatted_name = String::new();

    // remove excess spaces between words
    let words: Vec<&str> = name.split_whitespace().collect();

    // if let Some(first_name) = words.last() {
    //     for word in words.iter().take(words.len() - 1) {
    //         if let Some(first_char) = word.chars().next() {
    //             formatted_name.push(first_char.to_lowercase().next().unwrap());
    //         }
    //     }
    //     formatted_name.push_str(&first_name.to_lowercase());
    // }
    // => "nvmanh"

    if let Some(first_name) = words.last() { // get the firt_name (last element of words vec) eg. Nguyen Van Manh -> "Manh"
        // first, push the firstName into formatted_name
        formatted_name.push_str(&first_name.to_lowercase());

        // loop through the raminginh elements of words vec (skip the first_name):
        for word in words.iter().take(words.len() - 1) {
            // if there's still word left, take its first char to push
            if let Some(first_char) = word.chars().next() {
                formatted_name.push(first_char.to_lowercase().next().unwrap());
            }
        }
    }
    // => "manhnv"

    formatted_name
}

fn generate_email(name: &str, email_count: &mut HashMap<String, i32>) -> String {
    let formatted_name = format_name(name);

    let mut email = formatted_name.clone();
    let original_email = email.clone();

    // NOTE: calling `.to_owned()` or `.to_string()` will create a new owned instance of the email (mut) String,
    // so the original email variable is not moved, and we can still use it later to push the count.
    let mut count = *email_count.entry(email.to_owned()).or_insert(0);  // previously (email) -> moved, can't push later
    if count > 1 {
        // push to the count
        email.push_str(&count.to_string());
    }

    while email_count.contains_key(&email) {
        let new_count = format!("{}", count);
        let new_email = format!("{}{}", original_email, new_count);
        email = new_email;
        count += 1;
    }

    email.push_str("@gmail.com");
    //email.push_str("@gmail.com");
    //email_count.insert(email.clone(), 1);
    //count += 1;

    email.to_lowercase()
}

fn main() {
    /*
    // with io::stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Invalid number");

    let mut names: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        names.push(name.trim().to_string());
    }

    let mut email_count: HashMap<String, i32> = HashMap::new();
    for name in &names {
        let formatted_name = format_name(name);
        let email = generate_email(&formatted_name, &mut email_count);
        println!("{}", email);
    }

    for name in &names {
        println!("formatted name: {:?}", format_name(name));
    }
    */

    let mut email_count: HashMap<String, i32> = HashMap::new();
    let names = [
        "NguyEn     vAn manH",
        "NguyEn  thUy   LinH",
        "Nguyen   Vu    MANh",
        "NguYEN ThU    LinH",
        "HoANG    DinH   NaM",
        "NguyEN HuU    anH  tuAn",
        "NguYEn qUaN pHuC",
        "tRaN dUC tam",
    ];
    for name in &names {
        println!("formatted name: {:?}", format_name(name));
    }

    println!();

    for name in &names {
        let email = generate_email(name, &mut email_count);
        println!("{}", email);
        *email_count.entry(email).or_insert(1) += 1;
    }
}

/*
Input:

8
nguyEn     vAn manH
NguyEn  thUy   LinH
Nguyen   Vu    MANh
NguYEN ThU    LinH
HoANG    DinH   NaM
NguyEN HuU    anH  tuAn
NguYEn qUaN pHuC
tRaN dUC tam

Output:
manhnv1@gmail.com
linhnt1@gmail.com
manhnv2@gmail.com
linhnt2@gmail.com
namhd1@gmail.com
tuannha1@gmail.com
phucnq1@gmail.com
tamtd1@gmail.com

formatted name: "manhnv"
formatted name: "linhnt"
formatted name: "manhnv"
formatted name: "linhnt"
formatted name: "namhd"
formatted name: "tuannha"
formatted name: "phucnq"
formatted name: "tamtd"

*/