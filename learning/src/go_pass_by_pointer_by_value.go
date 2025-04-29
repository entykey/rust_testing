/*
Lời khuyên khi dùng Go
- Dùng con trỏ (*T) khi:

	+ Cần thay đổi giá trị gốc.

	+ Struct lớn (tránh sao chép tốn bộ nhớ).

- Dùng giá trị (T) khi:

	+ Không muốn thay đổi dữ liệu gốc (immutable).

	+ Struct nhỏ hoặc muốn đảm bảo an toàn goroutine.
*/

package main

import "fmt"

type User struct {
	Name string
}

func updateName(u *User, newName string) { // truyền bằng con trỏ
	u.Name = newName // Thay đổi giá trị gốc
}

func updateNameClone(u User, newName string) { // Truyền bằng giá trị
	// when u param is passed as a value, this func create a clone of it
	u.Name = newName // Thay đổi giá trị trên bản sao, không ảnh hưởng đến bản gốc
}

func main() {
	user := User{Name: "Alice"}
	updateName(&user, "Tuanhayho") // Truyền con trỏ
	fmt.Println(user.Name)         // Output: "Tuanhayho"

	updateNameClone(user, "Bob") // Truyền giá trị
	fmt.Println(user.Name)       // Output vẫn là "Tuanhayho"
}

// a Rust example for this Go code
/*
Output:
Original: User { name: "Alice" }
Updated (func): User { name: "Bob" }
Updated (method): User { name: "Charlie" }
*/
/*
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
*/
