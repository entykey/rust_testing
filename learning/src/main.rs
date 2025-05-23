// a Rust example for this Go code
/*
type User struct {
    Name string
}

// func updateName(u *User, newName string) {  // truyền bằng con trỏ
//     u.Name = newName // Thay đổi giá trị gốc
// }

func updateName(u User, newName string) { // Truyền bằng giá trị
    u.Name = newName    // Thay đổi giá trị trên bản sao, không ảnh hưởng đến bản gốc
}

func main() {
    user := User{Name: "Alice"}
    updateName(&user, "Bob") // Truyền con trỏ
    fmt.Println(user.Name)  // Output: "Bob"
} 
*/

/*
Output:
Original: User { name: "Alice" }
Updated (func): User { name: "Bob" }
Updated (method): User { name: "Charlie" }
*/
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

// Cách 1: Dùng function độc lập (Go-style)
fn update_name(u: User, new_name: String) -> User {
    User { name: new_name }
}

// Cách 2: Dùng method (Rust-style)
impl User {
    fn update_name_by_value(self, new_name: String) -> User {
        User { name: new_name }
    }
}

fn main() {
    let user: User = User { name: "Alice".to_string() };

    // Cách 1: Function độc lập
    let updated_user: User = update_name(user.clone(), "Bob".to_string());

    // Cách 2: Method
    let updated_user2: User = user.clone().update_name_by_value("Charlie".to_string());

    println!("Original: {:?}", user); // User { name: "Alice" }
    println!("Updated (func): {:?}", updated_user); // User { name: "Bob" }
    println!("Updated (method): {:?}", updated_user2); // User { name: "Charlie" }
}