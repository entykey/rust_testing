#[tokio::main]
async fn main() {
    use std::time::Duration;
    use rand;


    let mut handles = Vec::new();
    for i in 1..10 {
        let millis = rand::Rng::gen_range(&mut rand::thread_rng(), 1..100);  // cause err "moved"
        let r = tokio::spawn(async move {
            println!("hi number {} from the spawned thread!", i);
            // let _ = tokio::time::sleep(tokio::time::Duration::from_secs(i)).await;
            // let millis = rand::Rng::gen_range(&mut rand::thread_rng(), 1..100);  // cause err "moved"
            // tokio::time::sleep(Duration::from_millis(millis)).await;
            // tokio::time::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..100)));
            println!("bye number {} from the spawned thread!", i);
        });
        handles.push(r);
    }

    // let _ = tokio::join!(r);
    for i in handles {
        i.await.unwrap();
    }
}