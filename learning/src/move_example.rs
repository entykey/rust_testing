use tokio::time::{sleep, Duration};
use tokio::task;

async fn task_a() {
    sleep(Duration::from_secs(1)).await;
    println!("Task A done!");
}

async fn task_b(name: String) {
    sleep(Duration::from_secs(2)).await;
    println!("Task B: Hello, {}!", name);
}

#[tokio::main]
async fn main() {

    // first example of move:
    let name = "Alice".to_string();
    let age = 30;

    let closure = move || {
        println!("Name: {}", name);
        println!("Age: {}", age);
    };

    closure();
    // Here, the closure takes ownership of `name` and `age`,
    // so it can still be called even after `main` function ends.


    // 2nd example of tokio
    let name = "Bob".to_string();

    let handle_a = task::spawn(task_a());
    let handle_b = task::spawn(async move { // Remove `move` here, it still works
        task_b(name).await;
    });

    handle_a.await.unwrap();
    handle_b.await.unwrap();
}
