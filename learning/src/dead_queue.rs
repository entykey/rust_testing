use std::sync::Arc;
use tokio::time::{sleep, Duration};

const TASK_COUNT: usize = 1000;
const WORKER_COUNT: usize = 10;

// https://github.com/bikeshedder/deadqueue      (german quality)
// import crate:    $ cargo add deadqueue@0.2.4
type TaskQueue = deadqueue::limited::Queue<usize>;

#[tokio::main]
async fn main() {
    let queue = Arc::new(TaskQueue::new(TASK_COUNT));
    for i in 0..TASK_COUNT {
        queue.try_push(i).unwrap();
    }
    for worker in 0..WORKER_COUNT {
        let queue = queue.clone();
        tokio::spawn(async move {
            loop {
                let task = queue.pop().await;
                println!("worker[{}] processing task[{}] ...", worker, task);
            }
        });
    }
    while queue.len() > 0 {
        println!("Waiting for workers to finish...");
        sleep(Duration::from_millis(100)).await;
    }
    println!("All tasks done. :-)");
}