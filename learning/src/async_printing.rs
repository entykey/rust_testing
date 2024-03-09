use rand::seq::SliceRandom;
use std::time::Duration;
use tokio::time::sleep;
use tokio::runtime::Runtime;
use std::pin::Pin;
use std::future::Future;

async fn print_random_text() {
    let random_texts = [
        "Hello", "This", "Is", "An", "Example", "Of", "Printing", "Random", "Text", "One", "By", "One",
    ];

    for text in random_texts.choose_multiple(&mut rand::thread_rng(), random_texts.len()) {
        println!("{}", text);
        sleep(Duration::from_millis(100)).await;
    }
}

fn get_my_pods() -> impl Future<Output=()> {
    async {
        // Simulate an asynchronous operation
        sleep(Duration::from_secs(1)).await;
        println!("Get all my pods in default namespace");
    }
}

//#[tokio::main]
fn main() {
    //print_random_text().await;

    // create a runtime, this make main() no longer async method
    // we execute async tasks inside this runtime
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        print_random_text().await;
        get_my_pods().await;
        println!("all done!");
    });
}
