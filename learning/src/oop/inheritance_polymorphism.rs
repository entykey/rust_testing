trait Animal {
    fn speak(&self);
    fn custom_behavior(&self) {
        println!("Default behavior for Animal");
    }
}

struct Dog {
    name: String,
    breed: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }

    fn custom_behavior(&self) {
        println!("Custom behavior for Dog");
    }
}

struct Cat {
    name: String,
    breed: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }

    fn custom_behavior(&self) {
        println!("Custom behavior for Cat");
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
        breed: String::from("Golden Retriever"),
    };

    // Call custom_behavior method of Dog
    dog.custom_behavior(); // This will call the overridden custom_behavior method for Dog

    let cat = Cat {
        name: String::from("Whiskers"),
        breed: String::from("Siamese"),
    };

    // Call custom_behavior method of Cat
    cat.custom_behavior(); // This will call the overridden custom_behavior method for Cat
}
