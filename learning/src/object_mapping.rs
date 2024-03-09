// No Lifetime version
// #[derive(Debug)]
// struct Address {
//     street: String,
//     city: String,
//     state: String,
// }

// #[allow(non_snake_case)]
// #[derive(Debug)]
// struct User {
//     user_id: u32,
//     username: String,
//     email_address: String,
//     address: Address, // Nested Address struct
//     IsEmailConfirmed: u8,
// }

// #[derive(Debug)]
// struct UserViewModel {
//     id: u32,          // Maps to user_id
//     name: String,     // Maps to username
//     email: String,    // Maps to email_address
//     city: String, // Simplified address with only city
//     is_email_confirmed: bool,
// }


// fn map_users_to_viewmodels(users: Vec<User>) -> Vec<UserViewModel> {
//     users
//         .into_iter()
//         .map(|user| UserViewModel {
//             id: user.user_id,
//             name: user.username,
//             email: user.email_address,
//             city: user.address.city,
//             is_email_confirmed: user.IsEmailConfirmed == 1, // Check if it's 1
//         })
//         .collect()
// }

// fn main() {
//     let main_time = std::time::Instant::now();

//     // Create some fake User data
//     let users = vec![
//         User {
//             user_id: 1,
//             username: "user1".to_string(),
//             email_address: "user1@example.com".to_string(),
//             address: Address {
//                 street: "42-44 Cockburn St".to_string(),
//                 city: "Edinburgh EH1 1PB".to_string(),
//                 state: "State1".to_string(),
//             },
//             IsEmailConfirmed: 0,
//         },
//         User {
//             user_id: 2,
//             username: "user2 at Harry Potter Museum Context".to_string(),
//             email_address: "user2@example.com".to_string(),
//             address: Address {
//                 street: "40 Victoria St".to_string(),
//                 city: "Edinburgh EH1 2JW".to_string(),
//                 state: "State2".to_string(),
//             },
//             IsEmailConfirmed: 1,
//         },
//         User {
//             user_id: 3,
//             username: "user3".to_string(),
//             email_address: "user3@example.com".to_string(),
//             address: Address {
//                 street: "99 W Bow".to_string(),
//                 city: "Edinburgh EH1 2JP".to_string(),
//                 state: "State2".to_string(),
//             },
//             IsEmailConfirmed: 1,
//         },
//     ];

//     let viewmodels = map_users_to_viewmodels(users);
//     // Print the mapped data
//     println!("{:#?}", viewmodels);

//     // end of main
//     let duration: std::time::Duration = main_time.elapsed();
//     let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
//     let elapsed_seconds = elapsed_ms / 1000.0; // Convert milliseconds to seconds
//     println!("\nExecution time: {:?} ({:?} ms) ({:.8} s)", duration, elapsed_ms, elapsed_seconds);
// }



#[derive(Debug)]
struct Address<'a> {
    street: &'a str,
    city: &'a str,
    state: &'a str,
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct User<'a> {
    user_id: u32,
    username: &'a str,
    email_address: &'a str,
    address: Address<'a>, // Nested Address struct with generic lifetime
    IsEmailConfirmed: u8,
}

#[derive(Debug)]
struct UserViewModel<'a> {
    id: u32,          // Maps to user_id
    name: &'a str,    // Maps to username
    email: &'a str,   // Maps to email_address
    city: &'a str,    // Simplified address with only city
    is_email_confirmed: bool,
}

fn map_users_to_viewmodels<'a>(users: &Vec<User<'a>>) -> Vec<UserViewModel<'a>> {
    users
        .into_iter()
        .map(|user| UserViewModel {
            id: user.user_id,
            name: user.username,
            email: user.email_address,
            city: user.address.city,
            is_email_confirmed: user.IsEmailConfirmed == 1,
        })
        .collect()
}

fn main() {
    let main_time = std::time::Instant::now();

    // Create some fake User data
    let users = vec![
        User {
            user_id: 1,
            username: "user1",
            email_address: "user1@example.com",
            address: Address {
                street: "42-44 Cockburn St",
                city: "Edinburgh EH1 1PB",
                state: "State1",
            },
            IsEmailConfirmed: 0,
        },
        User {
            user_id: 2,
            username: "user2 at Harry Potter Museum Context",
            email_address: "user2@example.com",
            address: Address {
                street: "40 Victoria St",
                city: "Edinburgh EH1 2JW",
                state: "State2",
            },
            IsEmailConfirmed: 1,
        },
        User {
            user_id: 3,
            username: "user3",
            email_address: "user3@example.com",
            address: Address {
                street: "99 W Bow",
                city: "Edinburgh EH1 2JP",
                state: "State2",
            },
            IsEmailConfirmed: 1,
        },
    ];

    let viewmodels = map_users_to_viewmodels(&users);
    // Print the mapped data
    println!("{:#?}", viewmodels);

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_seconds = elapsed_ms / 1000.0; // Convert milliseconds to seconds
    println!(
        "\nExecution time: {:?} ({:?} ms) ({:.8} s)",
        duration,
        elapsed_ms,
        elapsed_seconds
    );
}
