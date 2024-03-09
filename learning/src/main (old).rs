// fn main() {
//     let a:u32 = 42;
//     let b = a;
//     let c:usize = 23;
//     let d = 'x';
//     let e = false;

//     println!("a: {}", std::mem::size_of_val(&a));
//     println!("b: {}", std::mem::size_of_val(&b));
//     println!("c: {}", std::mem::size_of_val(&c));
//     println!("d: {}", std::mem::size_of_val(&d));
//     println!("e: {}", std::mem::size_of_val(&e));
//   }

// use std::ops::Sub;

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // FILL in the blank in three ways: two of them use the default generic  parameters, the other one not.
// // Notice that the implementation uses the associated type `Output`.
// impl __ {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 });

//     println!("Success!");
// }

// fix:
// use std::ops::Sub;

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// implement subtract trait for Point
// impl<T: std::ops::Sub<Output = T>> Sub for Point<T>
// {
//     type Output = Point<T>;

//     fn sub(self, other: Point<T>) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("{:?}", Point { x: 2, y: 3 } - Point { x: 1, y: 0 });
//     println!("Success!");
// }





// https://qiita.com/lo48576/items/34887794c146042aebf1
// fn main() {
//     let vec_iter = (40..46).collect::<Vec<_>>().into_iter();

//     println!("first three:");
//     for (i, n) in vec_iter.take(3).enumerate() {
//         println!("{}: {}", i, n);
//     }

//     println!("rest:");
//     for (i, n) in vec_iter.enumerate() { // <- エラー！
//         // ↑ "use of moved value: `vec_iter` [E0382]"
//         println!("{}: {}", i, n);
//     }
// }

// fix:
// fn main() {
//     // イテレータは何度も使う(かつ状態が変更される)ので、mutableにする
//     let mut vec_iter: std::vec::IntoIter<i32> = (40..46).collect::<Vec<_>>().into_iter();

//     println!("first three:");
//     // `by_ref()` してから `take()` する
//     for (i, n) in vec_iter.by_ref().take(3).enumerate() {
//         println!("{}: {}", i, n);
//     }

//     println!("rest:");
//     for (i, n) in vec_iter.enumerate() { // <- おｋ
//         println!("{}: {}", i, n);
//     }
// }






// https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/98dc80
// use std::thread;
// use std::sync::Mutex;

// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3];

//     let handle: thread::JoinHandle<()> = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();

//     let m: Mutex<i32> = Mutex::new(5);

//     {
//         let mut num: std::sync::MutexGuard<'_, i32> = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}", m);
// }

// fn main() {
//     // raw &str: 16 bytes
//     // &str : 16 bytes
//     // String: 24 bytes
//     let a: &str = r#"This is a"#;
//     let b: &&str = &a;
//     let a_ref: Box<&&str> = Box::new(&a);
//     let arr: [i32;4] = [1,2,5,12];        // 16 bytes
//     let vec: Vec<i32> = vec![1,2,5,12];    // 24 bytes
//     //println!("a: {}", a);

//     println!("a is stored at {:p} and contains {}", &a, *&a);

//     println!("b (&a) is stored at {:p} and contains {}", b, *b);

//     println!("a_ref is stored at {:p} and contains {}", a_ref, a_ref);

//     println!("mem size of a: {}", std::mem::size_of_val(&*&a));

//     println!("mem size of b (&a): {}", std::mem::size_of_val(&*&b));

//     println!("mem size of a_ref: {}", std::mem::size_of_val(&*&a_ref));

//     println!("mem size of arr: {}", std::mem::size_of_val(&arr));

//     println!("arr is stored at {:p} and contains {:?}", &arr, arr);

//     println!("arr[0] is stored at {:p} and contains {}", &arr[0], *&arr[0]);
//     println!("arr[1] is stored at {:p} and contains {}", &arr[1], *&arr[1]);
//     println!("arr[2] is stored at {:p} and contains {}", &arr[2], *&arr[2]);
//     println!("arr[3] is stored at {:p} and contains {}", &arr[3], *&arr[3]);

//     println!("mem size of vec: {}", std::mem::size_of_val(&vec));

//     println!("vec is stored at {:p} and contains {:?}", &vec, vec);

//     println!("vec[0] is stored at {:p} and contains {}", &vec[0], *&vec[0]);
//     println!("vec[1] is stored at {:p} and contains {}", &vec[1], *&vec[1]);
//     println!("vec[2] is stored at {:p} and contains {}", &vec[2], *&vec[2]);
//     println!("vec[3] is stored at {:p} and contains {}", &vec[3], *&vec[3]);

// }





// https://stackoverflow.com/questions/71267423/handling-a-response-from-a-result-using-match
// fn use_result(par: i32) -> Result<i32, &'static str> {
//     if par == 0 {
//         println!("use_result(): error occured!");
//         Err("some error")
//     } else {
//         println!("par is 1");
//         Ok(par)
//     }
// }
// enum IntOrString {
//     Int(i32),
//     String(&'static str),
// }

// fn main() {
//     println!("method: Return an enum");
//     let i_or_s: IntOrString = match use_result(0) {
//         Ok(v) => IntOrString::Int(v),
//         Err(e) => IntOrString::String(e),
//     };

//     println!("method: Different code in each match arm");
//     match use_result(0) {
//         Ok(v) => {
//             println!("ok");
//         }
//         Err(e) => {
//             println!("err");
//         }
//     };

//     println!("method: panic");
//     let res: i32 = match use_result(0) {
//         Ok(v) => v,
//         Err(e) => panic!("cannot be zero"),
//       };
// }






// binary tree, ref: https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
// // use std::sync::Arc;
// // #[derive(PartialEq)]
// // struct Node<'a> {
// //     val: &'a str,
// //     l: Option<&'a Node<'a>>,        // added & as indirection
// //     // l: Option<Arc<Node<'a>>>,    // Smart pointers add overhead to performance !
// //     r: Option<&'a Node<'a>>,
// // }
// #[derive(Debug)]    // without this => binary operation `==` cannot be applied to type `Node<'_>`
// struct Node<'a, T: std::cmp::PartialOrd + std::cmp::PartialEq> {
//     val: &'a T,
//     l: Option<Box<Node<'a, T>>>,
//     r: Option<Box<Node<'a, T>>>,
// }
// // for generic, must specify PartialOrd
// impl<'a, T: std::cmp::PartialOrd> Node<'a, T> {
//     pub fn insert(&mut self, new_val: &'a T) {
//         if *self.val == *new_val {  // Node must have derived from PartialEq(string type) !
//             return
//         }
//         let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
//         match target_node {
//             &mut Some(ref mut subnode) => subnode.insert(new_val),
//             &mut None => {
//                 let new_node = Node { val: new_val, l: None, r: None };
//                 let boxed_node = Some(Box::new(new_node));
//                 *target_node = boxed_node;
//             }
//         }
//     }
// }
// fn main () {
//     let mut x = Node { val: &2, l: None, r: None };
//     x.insert(&4);
//     x.insert(&1);
//     x.insert(&8);
    
//     // assert!(x == Node {
//     //     val: &2,
//     //     l: Some(Box::new(Node {
//     //         val: &1,
//     //         l: None,
//     //         r: Some(Box::new(Node { val: &8, l: None, r: None })),
//     //     })),
//     //     r: Some(Box::new(Node { val: &4, l: None, r: None })),
//     // });
    
//     println!("{:?}", x);
// }






// async tokio runtime:
// use std::thread;
// use tokio::runtime::Handle; // 0.3.4

// #[tokio::main]
// async fn main() {
//     let threads: Vec<_> = (0..3)
//         .map(|thread_id| {
//             let handle = Handle::current();

//             thread::spawn(move || {
//                 eprintln!("Thread {} started", thread_id);

//                 for task_id in 0..3 {
//                     handle.spawn(async move {
//                         eprintln!("Thread {} / Task {}", thread_id, task_id);
//                     });
//                 }

//                 eprintln!("Thread {} finished", thread_id);
//             })
//         })
//         .collect();

//     for t in threads {
//         t.join().expect("Thread panicked");
//     }
// }






// https://www.snoyman.com/blog/2018/12/rust-crash-course-07-async-futures-tokio/
// use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
// use std::sync::Arc;
// use std::thread::{sleep, spawn};
// use std::time::Duration;

// #[derive(Clone)]
// pub struct Interval {
//     counter: Arc<AtomicUsize>,
//     still_running: Arc<AtomicBool>,
// }

// impl Drop for Interval {
//     fn drop(&mut self) {
//         println!("Interval thread shutting down");
//         self.still_running.store(false, Ordering::SeqCst);
//     }
// }

// impl Interval {
//     pub fn from_millis(millis: u64) -> Interval {
//         let duration = Duration::from_millis(millis);

//         let counter = Arc::new(AtomicUsize::new(0));
//         let counter_clone = counter.clone();

//         let still_running = Arc::new(AtomicBool::new(true));
//         let still_running_clone = still_running.clone();

//         spawn(move || {
//             println!("Interval thread launched");
//             while still_running_clone.load(Ordering::SeqCst) {
//                 sleep(duration);
//                 let old = counter_clone.fetch_add(1, Ordering::SeqCst);
//                 println!("Interval thread still alive, value was: {}", old);
//             }
//         });

//         Interval {
//             counter,
//             still_running,
//         }
//     }

//     pub fn get_counter(&self) -> usize {
//         self.counter.load(Ordering::SeqCst)
//     }
// }
// fn main() {
//     let interval: Interval = Interval::from_millis(500); // half a second
//     let duration: Duration = std::time::Duration::from_millis(100); // 2 seconds
//     // for i in 1..51 {
//     //     println!("Iteration number {}, counter is {}", i, interval.get_counter());
//     //     std::thread::sleep(duration);
//     // }

//     let mut last = interval.get_counter();
//     for i in 1..51 {
//         let curr = interval.get_counter();

//         if curr != last {
//             last = curr;
//             println!("Iteration number {}, counter is {}", i, curr);
//         }

//         std::thread::sleep(duration);
//     }
// }





// use tokio::time::{sleep, Duration};
// use std::pin::Pin;
// use std::future::Future;
// use std::task::{Context, Poll};

// struct MyFuture {
//     counter: usize,
// }

// impl MyFuture {
//     fn new() -> Self {
//         MyFuture { counter: 0 }
//     }
// }

// impl Future for MyFuture {
//     type Output = usize;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         // Simulate some asynchronous work
//         println!("Polling... {}", self.counter);

//         // Using tokio::time::sleep without async/await
//         let sleep = tokio::time::sleep(Duration::from_secs(1));
//         tokio::pin!(sleep);

//         // Update counter
//         self.counter += 1;

//         if self.counter < 5 {
//             Poll::Pending
//         } else {
//             Poll::Ready(self.counter)
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     let my_future = MyFuture::new();

//     // let result = my_future.await;

//     // // Specify the type of result
//     // let result: Result<usize, _> = my_future.await;

//     // match result {
//     //     Ok(value) => println!("Result: {}", value),
//     //     Err(_) => println!("Error occurred."),
//     // }

//     // Use await without Result
//     let result = my_future.await;
//     println!("Result: {}", result);

// }





// hashmap
// use std::collections::HashMap;
// use std::collections::hash_map::Iter;

// fn main() {
//     let mut my_map: HashMap<&str, &str> = HashMap::new();

//     my_map.insert("key1", "Tuấn hay ho");
//     my_map.insert("key2", "lắm");

//     // if let Some(value) = my_map.get("key1") {
//     //     println!("Value for key1: {}", value);
//     // }

//     println!("Sử dụng vòng lặp để duyệt qua tất cả các key trong HashMap:");
//     for (key, value) in &my_map {
//         println!("Value for {}: {}", key, value);
//     }

//     println!("Sử dụng iter() để lặp qua tất cả các cặp khóa-giá trị trong HashMap:");
//     for (key, value) in my_map.iter() {
//         println!("Value for {}: {}", key, value);
//     }


//     println!("Sử dụng .iter() để có iterator, và .next() để lặp qua các cặp khóa-giá trị:");
//     let mut iter: Iter<'_, &str, &str> = my_map.iter();

//     while let Some((key, value)) = iter.next() {
//         println!("Value for {}: {}", key, value);
//     }

//     // duplicated key will replace the existing one !!
//     my_map.insert("key2", "lắm lắm");
//     for (key, value) in &my_map {
//         println!("Value for {}: {}", key, value);
//     }

// }






// use std::collections::HashMap;

// // Một cấu trúc dữ liệu đơn giản để đại diện cho một mục trong `HashMap`
// #[allow(unused)]
// #[derive(Debug)]
// struct Item {
//     name: String,
//     value: i32,
// }

// fn main() {
//     // Khởi tạo một HashMap với kiểu String làm khóa và Item làm giá trị
//     let mut my_map: HashMap<String, Item> = HashMap::new();

//     // Thêm các mục vào HashMap
//     my_map.insert("item1".to_string(), Item { name: "Item 1".to_string(), value: 42 });
//     my_map.insert("item2".to_string(), Item { name: "Item 2".to_string(), value: 69 });
//     my_map.insert("item3".to_string(), Item { name: "Item 3".to_string(), value: 101 });

//     // Truy cập giá trị của một khóa và in ra
//     if let Some(item) = my_map.get("item1") {
//         println!("\nValue for item1: {:?}", item);
//     }

//     println!("\nLặp qua tất cả các cặp khóa-giá trị và in ra:");
//     for (key, value) in &my_map {
//         println!("Value for {}: {:?}", key, value);
//     }

//     // Sử dụng entry API để kiểm tra và thêm giá trị nếu khóa không tồn tại
//     my_map.entry("item4".to_string()).or_insert(Item { name: "Item 4".to_string(), value: 88 });

//     my_map.entry("item3".to_string()).or_insert(Item { name: "Item 3 nè".to_string(), value: 999 });

//     // Têm bất chấp
//     my_map.insert("item3".to_string(), Item { name: "Item 3 nè".to_string(), value: 999 });

//     println!("\nKiểm tra giá trị của khóa \"item4\":");
//     if let Some(item) = my_map.get("item4") {
//         println!("Value for item4: {:?}", item);
//     }

//     if my_map.contains_key("item3") {
//         if let Some(item) = my_map.get("item3") {
//             println!("\nWe've got item3 with val: {:?}", item.value);
//         }
//     }
//     println!("\n\"item3\": {:?}", my_map.get("item3"));


//     // Hiển thị toàn bộ HashMap sau khi thêm "item4"
//     println!("\nUpdated HashMap: {:#?}", my_map);
// }







// https://qiita.com/nisei275/items/2c5c6d934bdae5d138d1
// extern crate ws;
// extern crate env_logger;

// use chrono::{Local, DateTime};
// use ws::{listen, CloseCode, Sender, Handler, Handshake, Message, Result};

// fn main() {
//     // ロガーの初期化
//     env_logger::init();

//     // WebSocketの開始
//     listen("127.0.0.1:3012", |out| {
//         Server { 
//             out: out,
//             user_name: String::new()
//         }
//     }).unwrap();

//     struct Server {
//         out: Sender,
//         user_name: String
//     }

//     impl Handler for Server {
//         // WebSocketとのコネクション接続を開始した場合
//         fn on_open(&mut self, handshake: Handshake) -> Result<()> {
//             let hashed_key: String = handshake.request.hashed_key().unwrap();
//             let headers: &Vec<(String, Vec<u8>)> = handshake.request.headers();

//             // ヘッダーで送信されてきたユーザ名を取得する
//             for (k, v) in headers {
//                 if k == "User-Name" {
//                     self.user_name = String::from_utf8(v.to_vec()).unwrap();
//                 }
//             }

//             // Extracting the IP address from the Result<Option<String>, ws::Error>
//             let remote_ip: String = handshake.remote_addr()?.ok_or(ws::Error::new(ws::ErrorKind::Internal, "No remote address"))?;

//             // ログイン情報を接続している全てのクライアントに配信する
//             println!("[{}] {} Connected. Connection opened from ip: {} hash_key: {}", str_datetime(), self.user_name, remote_ip, hashed_key);
//             let send_message: String = format!("[{}] {} Join the Chat Room.", str_datetime(), self.user_name);

//             return self.out.broadcast(send_message);
//         }

//         // メッセージを受信した場合
//         fn on_message(&mut self, message: Message) -> Result<()> {
//             // 受信したメッセージを接続している全てのクライアントに配信する
//             let send_message: String = format!("[{}] {}: {}", str_datetime(), self.user_name, message);
//             println!("{}", send_message);
//             return self.out.broadcast(send_message);
//         }

//         // WebSocketとのコネクション接続が閉じた場合
//         fn on_close(&mut self, code: CloseCode, reason: &str) {
//             // ログイン情報を接続している全てのクライアントに配信する
//             println!("[{}] {} Disconnected for ({:?}) {}", str_datetime(), self.user_name, code, reason);
//             let send_message: String = format!("[{}] {} Left the Chat Room.", str_datetime(),self.user_name);
//             let _ = self.out.broadcast(send_message);
//         }
//     }

//     // 日付の文字列を取得
//     fn str_datetime() -> String {
//         // メッセージに日付を付与
//         let local_datetime: DateTime<Local> = Local::now();
//         let formatted_local_datetime: String = local_datetime.format("%Y-%m-%d %T").to_string();
//         return formatted_local_datetime;
//     }
// }






// // cursor tracking

// extern crate ws;
// extern crate env_logger;

// use std::thread;
// use std::sync::{Arc, Mutex};
// use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

// fn main() {
//     env_logger::init();

//     // Shared state for cursor position
//     let cursor_position = Arc::new(Mutex::new((0, 0)));

//     // WebSocket server
//     //     error code
//     // listen("127.0.0.1:3012", |out| {
//     //     let cursor_position = cursor_position.clone();

//     //     move |msg| {
//     //         handle_message(msg, out, cursor_position.clone())
//     //     }
//     // }).unwrap();
//     listen("127.0.0.1:3012", move |out| {
//         let cursor_position = cursor_position.clone();
    
//         move |msg| {
//             handle_message(msg, out.clone(), cursor_position.clone())
//         }
//     }).unwrap();
// }

// struct Server {
//     out: Sender,
//     cursor_position: Arc<Mutex<(i32, i32)>>,
// }

// impl Handler for Server {
//     fn on_open(&mut self, _: Handshake) -> Result<()> {
//         println!("WebSocket connection opened");
//         Ok(())
//     }

//     fn on_close(&mut self, code: CloseCode, reason: &str) {
//         println!("WebSocket closed ({:?}): {}", code, reason);
//     }
// }

// fn handle_message(msg: Message, out: Sender, cursor_position: Arc<Mutex<(i32, i32)>>) -> Result<()> {
//     match msg {
//         Message::Text(text) => {
//             if let Ok((x, y)) = serde_json::from_str::<(i32, i32)>(&text) {
//                 let mut position = cursor_position.lock().unwrap();
//                 *position = (x, y);
//                 println!("Cursor position: {:?}", *position);
//             }
//         }
//         _ => (),
//     }

//     Ok(())
// }



// // main.rs  (working now) (v1) (no user identity)

// extern crate ws;
// extern crate env_logger;

// use std::sync::{Arc, Mutex};
// use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

// fn main() {
//     env_logger::init();

//     // Shared state for cursor position
//     let cursor_position: Arc<Mutex<(i32, i32)>> = Arc::new(Mutex::new((0, 0)));

//     // WebSocket server
//     listen("127.0.0.1:3012", move |out| {
//         let cursor_position: Arc<Mutex<(i32, i32)>> = cursor_position.clone();

//         Server {
//             out,
//             cursor_position: cursor_position.clone(),
//         }
//     }).unwrap();
// }

// struct Server {
//     out: Sender,
//     cursor_position: Arc<Mutex<(i32, i32)>>,
// }

// impl Handler for Server {
//     fn on_open(&mut self, _: Handshake) -> Result<()> {
//         println!("WebSocket connection opened");
//         Ok(())
//     }

//     fn on_message(&mut self, msg: Message) -> Result<()> {
//         match msg {
//             Message::Text(text) => {
//                 println!("Received message: {}", text);
//             }
//             _ => (),
//         }

//         Ok(())
//     }

//     fn on_close(&mut self, code: CloseCode, reason: &str) {
//         println!("WebSocket closed ({:?}): {}", code, reason);
//     }
// }




/*
// Cursor tracking (server.rs)
extern crate ws;
extern crate env_logger;

use chrono::{Local, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

#[derive(Debug, Clone)]
struct Client {
    username: Option<String>,
    sender: Sender,
}

fn main() {
    env_logger::init();

    // Shared state for connected clients and cursor positions
    let connected_clients: Arc<Mutex<HashMap<Sender, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    // WebSocket server
    listen("127.0.0.1:3012", move |out| {
        let connected_clients: Arc<Mutex<HashMap<Sender, Client>>> = connected_clients.clone();

        Server {
            out,
            connected_clients: connected_clients.clone(),
        }
    }).unwrap();
}

struct Server {
    out: Sender,
    connected_clients: Arc<Mutex<HashMap<Sender, Client>>>,
}

impl Handler for Server {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let client = Client {
            username: None,
            sender: self.out.clone(),
        };

        let hashed_key: String = handshake.request.hashed_key().unwrap();
        let remote_addr: String = handshake.remote_addr()?.ok_or(ws::Error::new(ws::ErrorKind::Internal, "No remote address"))?;
        let connected_clients: Arc<Mutex<HashMap<Sender, Client>>> = self.connected_clients.clone();

        self.connected_clients.lock().unwrap().insert(self.out.clone(), client);
        println!("[{}] WebSocket connection opened from ip: {} hashedkey: {}", str_datetime(), remote_addr, hashed_key);
        println!("Connected clients: {:?}", connected_clients);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => {
                if let Ok(data) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&text) {
                    if let (Some(username), Some(x), Some(y)) = (
                        data.get("username").and_then(|u| u.as_str()),
                        data.get("x").and_then(|x| x.as_i64()),
                        data.get("y").and_then(|y| y.as_i64()),
                    ) {
                        let mut connected_clients: std::sync::MutexGuard<'_, HashMap<Sender, Client>> = self.connected_clients.lock().unwrap();

                        if let Some(client) = connected_clients.get_mut(&self.out) {
                            client.username = Some(username.to_string());
                            println!(
                                "Received message: {{\"username\":\"{}\", \"x\":{}, \"y\":{}}}",
                                username, x, y
                            );
                        }
                    }
                }
            }
            _ => (),
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let mut connected_clients: std::sync::MutexGuard<'_, HashMap<Sender, Client>> = self.connected_clients.lock().unwrap();

        if let Some(client) = connected_clients.remove(&self.out) {
            if let Some(username) = &client.username {
                println!("WebSocket closed for user {} ({:?}): {}", username, code, reason);
            }
        }
    }
}
// 日付の文字列を取得
fn str_datetime() -> String {
    // メッセージに日付を付与
    let local_datetime: DateTime<Local> = Local::now();
    let formatted_local_datetime: String = local_datetime.format("%Y-%m-%d %T").to_string();
    return formatted_local_datetime;
}

// output:
//   [2023-12-09 19:25:34] WebSocket connection opened from ip: 127.0.0.1 hashedkey: tJEZN20Lf4cHxBTejtYHrk9VItk=
//   Received message: {"username":"tuanhayho", "x":192, "y":128}
//   Connected clients: Mutex { data: {Sender { token: Token(0), channel: mio::channel::SyncSender<Command>, connection_id: 0 }: Client { username: None, sender: Sender { token: Token(0), channel: mio::channel::SyncSender<Command>, connection_id: 0 } }}, poisoned: false, .. }
*/




// Socket.io Nodejs chat main.rs

// extern crate ws;
// extern crate env_logger;

// use std::sync::{Arc, Mutex};
// use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

// // #[tokio::main]
// fn main() {
//     env_logger::init();

//     // Shared state for user names
//     let user_names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

//     // WebSocket server
//     listen("127.0.0.1:3012", move |out| {
//         let user_names = user_names.clone();

//         Server {
//             out,
//             user_names: user_names.clone(),
//         }
//     })
//     .unwrap();
// }

// struct Server {
//     out: Sender,
//     user_names: Arc<Mutex<Vec<String>>>,
// }

// impl Handler for Server {
//     fn on_open(&mut self, _: Handshake) -> Result<()> {
//         println!("WebSocket connection opened");
//         self.send_user_names();
//         Ok(())
//     }

//     fn on_message(&mut self, msg: Message) -> Result<()> {
//         match msg {
//             Message::Text(text) => {
//                 let data: serde_json::Value = serde_json::from_str(&text).unwrap();

//                 if let Some(username) = data.get("username") {
//                     let username = username.as_str().unwrap_or_default().to_string();
//                     self.user_names.lock().unwrap().push(username.clone());
//                     println!("pushing new username to the list: {}", *&username);
//                     self.send_user_names();
//                 }
//             }
//             _ => (),
//         }

//         Ok(())
//     }

//     fn on_close(&mut self, code: CloseCode, reason: &str) {
//         println!("WebSocket closed ({:?}): {}", code, reason);
//         self.send_user_names();
//     }
//     // fn on_close(&mut self, code: CloseCode, reason: &str) {
//     //     let mut connected_clients = self.connected_clients.lock().unwrap();
//     //     if let Some(client) = connected_clients.remove(&self.out.token()) {
//     //         if let Some(username) = &client.username {
//     //             println!("WebSocket closed for user {} ({:?}): {}", username, code, reason);
//     //         }
//     //     }
//     // }
// }

// impl Server {
//     async fn send_user_names(&self) {
//         let users = self.user_names.lock().unwrap().clone();
//         let message = serde_json::json!({
//             "get_users": true,
//             "users": users,
//         });

//         // Introduce a slight delay before sending the updated user list
//         std::thread::sleep(std::time::Duration::from_millis(100));

//         self.out.send(message.to_string()).unwrap();
//     }
// }








// // tokio example: https://docs.rs/tokio/1.35.0/tokio/task/fn.spawn.html
// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// use std::io;

// async fn my_background_op(id: i32) -> String {
//     let s: String = format!("Starting background task {}.", id);
//     println!("{}", s);
//     // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//     s
// }

// async fn a_single_background_task() {
//     let s: String = format!("Starting single background task");
//     println!("{}", s);
//     tokio::time::sleep(std::time::Duration::from_secs(5)).await;

//     println!("Single background task finished");
// }

// async fn process(socket: &mut TcpStream) -> io::Result<String> {
//     let mut buf: [u8; 1024] = [0; 1024];
//     let bytes_read: usize = socket.read(&mut buf).await?;

//     if bytes_read == 0 {
//         // Connection closed
//         return Ok("Failed".to_string());    //  Ok(())
//     }

//     // background jobs: To run multiple tasks in parallel and receive their results, join handles can be stored in a vector.
//     // This example pushes the tasks to outputs in the order they were started in. If you do not care about the ordering of 
//     // the outputs, then you can also use a JoinSet.
//     let ops: Vec<i32> = vec![1, 2, 5, 3];
//     let mut tasks: Vec<tokio::task::JoinHandle<String>> = Vec::with_capacity(ops.len());

//     for op in ops {
//         // This call will make them start running in the background
//         // immediately.
//         println!("pushing op {} to task", op);
//         tasks.push(tokio::spawn(my_background_op(op)));
//     }
    
//     let mut single_background_tasks: Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(1);
//     single_background_tasks.push(tokio::spawn(a_single_background_task()));


//     // Collect the results separately without calling await in this loop
//     let mut outputs: Vec<String> = Vec::with_capacity(tasks.len());

//     // Await the tasks in a separate loop
//     for task in tasks {
//         println!("pushing task {:?} to awaited outputs", task);
//         outputs.push(task.await.unwrap());
//     }
//     println!("{:?}", outputs);
//     // end of background jobs



//     let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello, bytes read: {}", *&bytes_read);

//     // Write the response to the socket
//     socket.write_all(response.as_bytes()).await?;

//     Ok("success".to_string())
// }

// async fn process1 (mut socket: TcpStream) -> io::Result<()> {
//     use tokio::task;
//     use tokio::time;
//     use std::time::Duration;

//     let original_task: task::JoinHandle<()> = task::spawn(
//         async {
//             let _detached_task: task::JoinHandle<()> = task::spawn(
//                 async {
//                 // Here we sleep to make sure that the first task returns before.
//                 time::sleep(Duration::from_millis(10)).await;
//                 // This will be called, even though the JoinHandle is dropped.
//                 println!("♫ Still alive ♫");
//             });
//             let _detached_task1: task::JoinHandle<()> = task::spawn(
//                 async {
//                 // Here we sleep to make sure that the first task returns before.
//                 // time::sleep(Duration::from_millis(10)).await;
//                 // This will be called, even though the JoinHandle is dropped.
//                 println!("♫ Task1 Still alive ♫");
//             });
//     });

//     original_task.await.expect("The task being joined has panicked");
//     println!("Original task is joined.");

//     // We make sure that the new task has time to run, before the main
//     // task returns.

//     time::sleep(Duration::from_millis(1000)).await;

//     let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello from process1");

//     // Write the response to the socket
//     socket.write_all(response.as_bytes()).await?;
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await?;

//     // std::net::TcpListener instead of tokio::net::TcpListener
//     let std_listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
//     std_listener.set_nonblocking(true)?;
//     let listener = TcpListener::from_std(std_listener)?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             println!("Now moved to process spawning block");

//             // Process each socket concurrently.
//             // if let Err(e) = process(&mut socket).await {
//             //     eprintln!("Error processing connection: {}", e);
//             // }
            
//             match process(&mut socket).await {
//                 Ok(result) => { // Ok(_) => {...}
//                     println!("Process succeeded: {:?}", result);
//                 }
//                 Err(e) => {
//                     eprintln!("Error processing connection: {}", e);
//                 }
//             }

//             // if let Err(e) = process1(socket).await {
//             //     eprintln!("Error processing connection: {}", e);
//             // }
//         });
//     }


    
//     loop {  // without loop, the server will stop after a client connected to it.
//         // eg1.
//         // match listener.accept().await {
//         //     Ok((_socket, addr)) => println!("new client: {:?}", addr),
//         //     Err(e) => println!("couldn't get client: {:?}", e),
//         // }
        
//         // eg2. (Only spawn one log)
//         // let (mut socket, addr) = listener.accept().await?;
//         // tokio::spawn(async move {
//         //     println!("new client: {:?}", addr);
//         // });

//         // eg3. (Similar to eg2 but use result<()> so it requires move to function)
//         // let (mut socket, addr) = listener.accept().await?;
//         // tokio::spawn(async move {print_client(socket, addr).await.unwrap()});

//         // eg4. (got duplicated log (2 ip addr for 1 client) )
//         // if let Ok((mut socket, addr)) = listener.accept().await {
//         //     tokio::spawn(handle_connection(socket, addr));
//         // }


//         // eg5. (same as eg4.)
//         // let (mut socket, addr) = listener.accept().await?;
//         // tokio::spawn(async move {
//         //     let mut buf = [0; 1024];

//         //     // In a loop, read data from the socket and write the data back.
//         //     loop {
//         //         let n = match socket.read(&mut buf).await {
//         //             // socket closed
//         //             Ok(0) => {
//         //                 // socket closed
//         //                 return;
//         //             }
//         //             Ok(n) => {
//         //                 println!("new client: {:?}", addr);
//         //                 let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello from process1");
        
//         //                 // Write the response to the socket
//         //                 if let Err(e) = socket.write_all(response.as_bytes()).await {
//         //                     eprintln!("failed to write to socket; err = {:?}", e);
//         //                     return;
//         //                 }
        
//         //                 // Write the data back
//         //                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//         //                     eprintln!("failed to write to socket; err = {:?}", e);
//         //                     return;
//         //                 }
//         //             }
//         //             Err(e) => {
//         //                 eprintln!("failed to read from socket; err = {:?}", e);
//         //                 return;
//         //             }
//         //         };
//         //     }
//         // });
        
//     }
    


//     // JoinHandle example: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
//     // let join_handle: tokio::task::JoinHandle<Result<i32, io::Error>> = tokio::spawn(async {
//     //     Ok(5 + 3)
//     // });

//     // let result: i32 = join_handle.await??;
//     // println!("{:?}", result);
//     // assert_eq!(result, 8);
//     // Ok(())


    
//     Ok(())




//     // cancel example: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort_handle
//     // let mut handles: Vec<tokio::task::JoinHandle<bool>> = Vec::new();

//     // handles.push(tokio::spawn(async {
//     //     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
//     //     true
//     // }));

//     // handles.push(tokio::spawn(async {
//     //     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
//     //     false
//     // }));

//     // let abort_handles: Vec<tokio::task::AbortHandle> = handles.iter().map(|h| h.abort_handle()).collect();

//     // for handle in abort_handles {
//     //     handle.abort();
//     // }

//     // for handle in handles {
//     //     // assert!(*&handle.await.unwrap_err().is_cancelled());
//     //     println!("is_cancelled: {}", handle.await.unwrap_err().is_cancelled());
//     // }
//     // Ok(())

// }
// async fn print_client(mut socket: tokio::net::TcpStream, addr: std::net::SocketAddr) -> Result<(), std::io::Error> {
//     println!("new client: {:?}", addr);
//     Ok(())
// }
// async fn handle_connection(mut socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
//     let mut buf = [0; 1024];

//     // In a loop, read data from the socket and write the data back.
//     loop {
//         match socket.read(&mut buf).await {
//             Ok(0) => {
//                 // socket closed
//                 return;
//             }
//             Ok(n) => {
//                 println!("new client: {:?}", addr);
//                 let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello from process1");

//                 // Write the response to the socket
//                 if let Err(e) = socket.write_all(response.as_bytes()).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }

//                 // Write the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//             Err(e) => {
//                 eprintln!("failed to read from socket; err = {:?}", e);
//                 return;
//             }
//         }
//     }
// }


// // use async_std::{
// //     prelude::*, // 1
// //     task, // 2
// //     net::{TcpListener, ToSocketAddrs}, // 3
// // };

// // type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // 4





// tokio example: https://docs.rs/tokio/1.35.0/tokio/task/fn.spawn.html
// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// use std::io;

// async fn a_single_background_task() {
//     let s: String = format!("[*] Starting single background task");
//     println!("{}", s);
//     tokio::time::sleep(std::time::Duration::from_secs(5)).await;

//     println!("[*] Single background task finished");
// }

// async fn process(socket: &mut TcpStream) -> io::Result<String> {
//     let mut buf: [u8; 1024] = [0; 1024];
//     let bytes_read: usize = socket.read(&mut buf).await?;

//     if bytes_read == 0 {
//         // Connection closed
//         return Ok("Failed".to_string());    //  Ok(())
//     }

//     tokio::spawn(a_single_background_task());

//     // response to client now while still continue running the background task
//     let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello, bytes read: {}", *&bytes_read);

//     // Write the response to the socket
//     socket.write_all(response.as_bytes()).await?;

//     Ok("success".to_string())
// }


// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await?;

//     // std::net::TcpListener instead of tokio::net::TcpListener
//     let std_listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
//     std_listener.set_nonblocking(true)?;
//     let listener = TcpListener::from_std(std_listener)?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             // println!("Now moved to process spawning block");

//             // Process each socket concurrently.
            
//             match process(&mut socket).await {
//                 Ok(result) => { // Ok(_) => {...}
//                     // println!("Process succeeded: {:?}", result);
//                 }
//                 Err(e) => {
//                     eprintln!("Error processing connection: {}", e);
//                 }
//             }
//         });
//     }
        

//     Ok(())

// }





// https://stackoverflow.com/questions/76110194/execute-a-long-running-task-in-background-and-show-progress-until-its-not-compl
// use std::io::Write;
// use core::time::Duration;
// #[tokio::main]
// async fn test() {
//     let h: tokio::task::JoinHandle<u8> = tokio::task::spawn(update_device());
//     while !h.is_finished() {
//         tokio::time::sleep(Duration::from_millis(200)).await;
//         print!(".");
//         std::io::stdout().flush().unwrap();
//     }
//     println!();
//     if h.await.unwrap() == 0 {
//         println!("Update completed successfully");
//     } else {
//         println!("Update failed");
//     }

// }

// async fn update_device() -> u8 {
//     println!("Updating the driver");
//     tokio::time::sleep(Duration::from_secs(2)).await;
//     0
// }

// fn main() {
//     test()
// }





// use std::error::Error;
// use tokio::time::Duration;
// use std::io::Write;
// #[tokio::main]
// async fn main() {
//     tokio::select! {
//         _ = do_actual_update_async() => {
//             println!("\ndriver installation completed")
//         }
//         _ = looper() => {
//             panic!("never come here")
//         }
//     };
//     println!("Completed");
// }
// async fn do_actual_update_async() -> Result<(), Box<dyn Error + Send + Sync>> {
//    println!("Updating the driver.......\n");
//     let h = tokio::task::spawn(async { update_device().await });
//     h.await?
// }

// async fn looper() -> Result<(), Box<dyn Error + Send + Sync>> {
//     loop {
        
//         tokio::time::sleep(Duration::from_secs(2)).await;
//         print!(".");
//         std::io::stdout().flush().unwrap();
//     }
// }

// async fn update_device() -> Result<(), Box<dyn Error + Send + Sync>> {
//     // actual implementation of update device
//     tokio::time::sleep(Duration::from_secs(20)).await;
//     Ok(())
// }





// use std::sync::Arc;
// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::sync::Mutex;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let should_start_update: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

//     let should_start_update_clone: Arc<Mutex<bool>> = Arc::clone(&should_start_update);

//     let h: tokio::task::JoinHandle<u8> = tokio::spawn(update_device(should_start_update_clone));

//     // Create a TcpListener
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//     println!("Server listening on 127.0.0.1:8080");

//     while let Ok((mut socket, _)) = listener.accept().await {
//         let should_start_update_clone = Arc::clone(&should_start_update);
//         tokio::spawn(async move {
//             // Process each socket concurrently.
//             match process(&mut socket, should_start_update_clone).await {
//                 Ok(result) => {
//                     // Ok(_) => {...}
//                     // println!("Process succeeded: {:?}", result);
//                 }
//                 Err(e) => {
//                     eprintln!("Error processing connection: {}", e);
//                 }
//             }
//         });
//     }

//     // Wait for the update task to complete
//     let result = h.await.unwrap();

//     println!();
//     if result == 0 {
//         println!("Update completed successfully");
//     } else {
//         println!("Update failed");
//     }

//     Ok(())
// }

// async fn update_device(should_start_update: Arc<Mutex<bool>>) -> u8 {
//     {
//         let mut guard = should_start_update.lock().await;
//         if !*guard {
//             // Another task has already started the update, so return immediately
//             return 0;
//         }
//         // Set the flag to false, indicating that the update task is starting
//         *guard = false;
//     }

//     println!("Updating the driver");
//     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

//     // Simulate a long-running task
//     // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

//     println!("Done updating the driver");

//     // Set the flag back to true, indicating that the update task has finished
//     *should_start_update.lock().await = true;

//     0
// }

// async fn process(socket: &mut TcpStream, should_start_update: Arc<Mutex<bool>>) -> Result<String, Box<dyn std::error::Error>> {
//     let mut buf: [u8; 1024] = [0; 1024];
//     let bytes_read: usize = socket.read(&mut buf).await?;

//     if bytes_read == 0 {
//         // Connection closed
//         return Ok("Failed".to_string());
//     }

//     let background_task_result = update_device(should_start_update.clone()).await;

//     // Response to the client now while still continuing to run the background task
//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello, bytes read: {} Background task result: {}",
//         bytes_read, background_task_result
//     );

//     // Write the response to the socket
//     socket.write_all(response.as_bytes()).await?;

//     Ok("success".to_string())
// }






/*
// documented
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
extern crate futures;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let should_start_update: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    let should_start_update_clone: Arc<Mutex<bool>> = Arc::clone(&should_start_update);

    // this below line will automatically spawn the task when server started
    let h: tokio::task::JoinHandle<()> = tokio::spawn(async {
        for i in 1..=10 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("I always run once when the server started! {}", i);
        }
    });

    // Create a TcpListener (tokio)
    let listener: tokio::net::TcpListener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    while let Ok((mut socket, _)) = listener.accept().await {
        let should_start_update_clone: Arc<Mutex<bool>> = Arc::clone(&should_start_update);
        tokio::spawn(async move {
            // Immediately respond to the client
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: 55\r\n\r\nImmediate response, my tasks still run in background :>");

            // Write the response to the socket
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Error writing to socket: {}", e);
            }

            // Spawn a separate task for the background update
            tokio::spawn(async move {

                // the below task not getting duplicated
                // let background_task_result = futures::executor::block_on(update_device(should_start_update_clone.clone()));
                // let background_task_result = update_device(should_start_update_clone.clone()).await;
                let _ = update_device(should_start_update_clone.clone()).await;

                
                // but the belows line does
                
                // if background_task_result == 0 {
                //     println!("Task created successfully");
                // } else {
                //     println!("Task creation failed");
                // }

                // println!("result: {}", background_task_result);

            });
        });
    }

    Ok(())
}

async fn update_device(should_start_update: Arc<Mutex<bool>>) -> u8 {
    // check if there is another task trying to update device
    {
        let mut guard: tokio::sync::MutexGuard<'_, bool> = should_start_update.lock().await;
        if !*guard {
            // Another task has already started the update, so return immediately
            println!("already being updated by another task");
            return 0;
        }
        // Set the flag to false, indicating that the update task is starting
        *guard = false;
    }

    println!("[update_device] Updating the driver");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Simulate a long-running task
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("[update_device] Done updating the driver");

    // Set the flag back to true, indicating that the update task has finished
    *should_start_update.lock().await = true;

    0
}
*/






// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
// use futures::executor::block_on;
// async fn hello_world() {
//     println!("hello, world!");
// }
// fn main() {
//     let future = hello_world(); // Nothing is printed
//     block_on(future); // `future` is run and "hello, world!" is printed
// }




/*
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const NUM_WORKERS: u8 = 4;
const NUM_TASKS: u8 = 20;

fn main() {
    let (task_tx, task_rx) = mpsc::channel();
    let task_rx: Arc<Mutex<mpsc::Receiver<usize>>> = Arc::new(Mutex::new(task_rx));

    let (result_tx, result_rx) = mpsc::channel();

    // Create worker threads
    for id in 0..NUM_WORKERS {
        let task_rx: Arc<Mutex<mpsc::Receiver<usize>>> = task_rx.clone();
        let result_tx: mpsc::Sender<String> = result_tx.clone();

        thread::spawn(move || {
            println!("Worker {} is waiting for tasks.", id);

            loop {
                // Get the next task from the task channel
                let task = match task_rx.lock().unwrap().recv() {
                    Ok(task) => task,
                    Err(_) => {
                        // The channel is closed, so the worker should exit
                        println!("Worker {} is exiting.", id);
                        break;
                    }
                };

                // Process the task
                println!("Worker {} is processing task: {}", id, task);

                // Simulate some work
                thread::sleep(std::time::Duration::from_millis(500));

                // Send the result back to the result channel
                result_tx.send(format!("Result of task {}: done", task)).unwrap();
            }
        });
    }

    // Submit tasks to the worker pool
    for task_id in 0..NUM_TASKS {
        task_tx.send(task_id.into()).unwrap();
    }

    // Close the task channel to signal the workers to exit
    drop(task_tx);

    // Collect results from the workers
    for _ in 0..NUM_TASKS {
        let result: String = result_rx.recv().unwrap();
        println!("Received result: {}", result);
    }
}
*/




/*
use std::sync::{Arc, Mutex};
use std::{thread, time};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        let delay = time::Duration::from_millis(1000);

        thread::sleep(delay);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });                                         


    let philosophers = vec![
        Philosopher::new("Donald", 0, 1),
        Philosopher::new("Larry", 1, 2),
        Philosopher::new("Mark", 2, 3),
        Philosopher::new("John", 3, 4),
        Philosopher::new("Bruce", 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
*/




/*
// https://www.dotnetperls.com/hashmap-rust
// BTreeMap is faster than HashMap
use std::collections::*;
use std::time::*;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("bird", 0);
    map.insert("frog", 0);  // try comment this, and it prints "NOT FOUND"
    map.insert("dog", 0);

    let mut bt: BTreeMap<&str, i32> = BTreeMap::new();
    bt.insert("bird", 0);
    bt.insert("frog", 0);
    bt.insert("dog", 0);

    if let Ok(max) = "1000000".parse::<usize>() { // 100000000
        let mut count = 0;

        // Version 1: use HashMap. (This version of the code tries to access the "frog" key of the HashMap repeatedly.)
        let t0 = Instant::now();
        for _ in 0..max {
            match map.get("frog") {
                Some(result) => count += 1,
                None => println!("NOT FOUND")
            }
        }
        println!("HashMap: {} ms", t0.elapsed().as_millis());

        // Version 2: use BTreeMap. (Here we do the same thing as the HashMap code, but with a BTreeMap instead—the logic is the same.)
        let t1 = Instant::now();
        for _ in 0..max {
            if let Some(result) = bt.get("frog") {
                count += 1;
            }
        }
        println!("BTreeMap: {} ms", t1.elapsed().as_millis());
        println!("{}", count);
    }
}
*/






// https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html
// use tokio::sync::broadcast;

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx1) = broadcast::channel(16);
//     let mut rx2: broadcast::Receiver<i32> = tx.subscribe();

//     tokio::spawn(async move {
//         // assert_eq!(rx1.recv().await.unwrap(), 10);
//         // assert_eq!(rx1.recv().await.unwrap(), 20);
//         println!("{}", rx1.recv().await.unwrap());
//         println!("{}", rx1.recv().await.unwrap());
//     });

//     tokio::spawn(async move {
//         // assert_eq!(rx2.recv().await.unwrap(), 10);
//         // assert_eq!(rx2.recv().await.unwrap(), 20);
//         println!("{}", rx2.recv().await.unwrap());
//         println!("{}", rx2.recv().await.unwrap());
//     });

//     // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

//     tx.send(10).unwrap();
//     tx.send(20).unwrap();
// }






/* 
// https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html
// by this approach of multiple `tokio::spawn()`, we create 4 concurrent async tasks that run toghether
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender1, mut receiver) = mpsc::channel::<String>(10); // Specify the type as String
    let sender2: mpsc::Sender<String> = sender1.clone();
    let sender3 = sender1.clone();
    let sender4 = sender1.clone();

    println!("max channel capacity: {}", &sender1.max_capacity());  // no definition in struct `UnboundedSender`

    // timer to measure 4 spawned concurrent tasks
    let time: std::time::Instant = std::time::Instant::now();

    let handle1 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        for i in 0..=10 {
            if let Err(_) = sender1.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
            // task waits until the receiver receives a value.
        }
    });

    let handle2 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        for i in 11..=20 {
            if let Err(_) = sender2.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
    }).await.unwrap();


    let handle3 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        for i in 21..=30 {
            if let Err(_) = sender3.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    // tokio::join!(handle1, handle2, handle3);


    tokio::spawn(async move {
        // This will return an error and send
        // no message if the buffer is full
        let _ = sender4.try_send("from sender4".to_string());
    });

    while let Some(i) = receiver.recv().await {
        println!("got message = {}", i);
    }

    // Stop timer & print to terminal
    let duration: std::time::Duration = time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_seconds: f64 = elapsed_ms / 1000.0;
    println!(
        ">>>  time elapsed: {:?} ({:?} ms) ({:.8} s)",
        duration, elapsed_ms, elapsed_seconds
    );

}
*/





/*
// unbounded sender:
use tokio::sync::mpsc;

#[derive(Debug)]
enum Message {
    Integer(i32),
    Float(f64),
    Text(String),
}

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::unbounded_channel::<Message>();

    let sender_clone = sender.clone();

    tokio::spawn(async move {
        let _ = sender_clone.send(Message::Integer(42));
    });

    tokio::spawn(async move {
        let _ = sender.send(Message::Text("Hello, world!".to_string()));
    });

    while let Some(msg) = receiver.recv().await {
        match msg {
            Message::Integer(i) => println!("Received integer: {}", i),
            Message::Float(f) => println!("Received float: {}", f),
            Message::Text(s) => println!("Received text: {}", s),
        }
    }
}
*/




/*
// #![allow(unused)]
// retrieving available resources:
// use num_cpus;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};

fn main() {

    // Create a new System instance to get the RAM info
    let mut system = System::new_all();

    // Retrieve the total RAM in megabytes
    let total_ram_mb = system.total_memory() / (1024 * 1024);
    println!("Total RAM: {} MB", total_ram_mb);

    // RAM and swap information:
    let interval = std::time::Duration::from_millis(600);
    let mut next_time = std::time::Instant::now() + interval;

    loop {
        // First we update all information of our `System` struct.
        system.refresh_all();

        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let memory_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
        // println!("total memory: {} bytes", total_memory);
        // println!("used memory : {} bytes", used_memory);
        println!("memory used : {:.2}%", memory_percentage);
        // println!("total swap  : {} bytes", system.total_swap());
        // println!("used swap   : {} bytes", system.used_swap());

        std::thread::sleep(next_time - std::time::Instant::now());
        next_time += interval;
    }
    
}

*/




/*
use std::sync::{Arc, Mutex};
use sysinfo::{System, SystemExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
// use futures_util::sink::SinkExt;    //  ws_stream.send(message).await.expect("Error sending message to client");
                                    // |               ^^^^ method not found in `&WebSocketStream<TcpStream>`
use futures_util::{future, StreamExt, SinkExt, TryStreamExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    println!("WebSocket server is running on ws://127.0.0.1:8080");

    let system = Arc::new(Mutex::new(System::new_all()));
    let interval = Arc::new(Mutex::new(interval(Duration::from_millis(600))));

    while let Ok((stream, _)) = listener.accept().await {
        let system_clone = Arc::clone(&system);
        let interval_clone = interval.clone();
        tokio::spawn(handle_connection(stream, system_clone, interval_clone));
    }
}

async fn handle_connection(mut stream: TcpStream, system: Arc<Mutex<System>>, interval: Arc<Mutex<tokio::time::Interval>>) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");

    // Send initial memory percentage to the client
    send_memory_percentage(ws_stream, &system).await;

    // Periodically send memory percentage updates
    // while let Some(_) = interval.lock().unwrap().next().await {
        // send_memory_percentage(ws_stream, &system).await;
    // }
}

async fn send_memory_percentage(mut ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>, system: &Arc<Mutex<System>>) {
    let system: std::sync::MutexGuard<'_, System> = system.lock().unwrap();
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let memory_percentage = (used_memory as f64 / total_memory as f64) * 100.0;

    let message = Message::Text(format!("Memory used: {:.2}%", memory_percentage));
    let (tx, rx) = unbounded();

    let (write, read) = ws_stream.split();
    ws_stream.send(message).await.expect("Error sending message to client");
}
*/


/*
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, StreamExt, SinkExt, TryStreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = futures_channel::mpsc::UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<std::net::SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    println!("WebSocket server is running on ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_map_clone = peer_map.clone();
        tokio::spawn(handle_connection(stream, addr, peer_map_clone));
    }
}

async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr, peer_map: PeerMap) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("Client connected from: {}", addr);

        let (tx, rx) = unbounded();

        // Insert the sender into the PeerMap
        // let peer_addr = addr;
        peer_map.lock().unwrap().insert(addr, tx);

        let (write, read) = ws_stream.split();

        let broadcast_incomming = read.try_for_each(|msg| {
            println!("Received message from {}: {}", addr, msg.to_text().unwrap());

            let peers: std::sync::MutexGuard<'_, HashMap<std::net::SocketAddr, UnboundedSender<Message>>> = peer_map.lock().unwrap();

            // broadcast message to everyone except the sender.
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);

            for recp in broadcast_recipients {
                recp.unbounded_send(msg.clone()).unwrap();
            }

            futures_util::future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(write);

        futures_util::pin_mut!(broadcast_incomming, receive_from_others);
        future::select(broadcast_incomming, receive_from_others).await;

        println!("{} disconnected", &addr);
        peer_map.lock().unwrap().remove(&addr);

    }
}
*/



// use std::thread;
//     fn main() {
//         for i in 1..10 {
//             let handle = thread::spawn(move || {
//                 println!("Hello from thread number {}", i);
//             });
//             let _ = handle.join();
//         }
// }






/*
#[allow(unused)]
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    // Receiver must be mutable in order to call ".recv()"
    let (tx, rx) = mpsc::channel::<i32>(10);

    // (By wrapping the original receiver `rx` in an Arc and a Mutex, `rx` does not need to be mutable anymore)
    // * Explain: The Arc allows you to share ownership of `rx` across multiple tasks, 
    // * and the Mutex ensures that only one task at a time can have mutable access to `rx`. 
    // * After this line, `rx` is no longer mutable, and you can safely share it among asynchronous tasks.
    // `tokio::sync::Mutex`  or  `futures::lock::Mutex`  both works
    let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));
    let rx_clone = rx.clone();


    // Using Sink trait to send values
    tokio::spawn(async move {
        // tokio::time::sleep(Duration::from_micros(10)).await;
        tx.send(42).await.unwrap();
        tx.send(99).await.unwrap();
    });

    
    // Using tokio::select! to concurrently handle both recv and try_recv
    // tokio::select! {
    //     // _ = tokio::spawn(async move {
    //     //     while let Some(value) = rx.recv().await {
    //     //         println!("[recv] Received: {}", value);
    //     //     }
    //     // }) => {}

    //     // since try_recv() is non-async operation, 
    //     // it does not return a Future, so it cannot be used directly within the tokio::select! macro, which expects futures.
    //     // That's why we must be wrapped with `spawn_blocking()` to fit within `tokio::select()`
    //     // And here it does not even wait for sleep duration, but terminate the program right away.
    //     // Only by removing the `tokio::time::sleep(...).await`, it will receive the message
    //     _ = tokio::task::spawn(async move {
    //         while let Ok(value) = rx.lock().await.try_recv() {
    //             println!("[try_recv] Received: {}", value);
    //         }
    //     }) => {}
    // }
    
    

    // Using Mutex lock to receive values with recv()
    tokio::spawn(async move {
        // loop {
            match rx_clone.lock().await.recv().await {
                Some(value) => {
                    println!("[recv match] Received: {}", value);
                }
                None => {
                    println!("[recv match] Received: None");
                }
            }
        // }
    }).await.unwrap();  // without this `.await.unwrap()` at tail, the program terminate immediately !


    // Using Mutex lock to receive values with recv()
    tokio::spawn(async move {
        // If the recv() returns None, indicating that the channel is closed, the while let loop will terminate.
        while let Some(value) = rx.lock().await.recv().await {
            println!("[recv while let] Received: {}", value);
        }
        println!("[recv while let] Channel closed");    // without this, it just skip
    }).await.unwrap();


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/





// try using RwLock
/*
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::Duration;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<i32>(10);
    let rx = Arc::new(RwLock::new(rx));
    let tx = Arc::new(Mutex::new(tx));
    let rx_clone = Arc::clone(&rx);

    // Using Sink trait to send values
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_micros(10)).await;
        tx.lock().await.send(42).await.unwrap();
        tx.lock().await.send(99).await.unwrap();
    });

    // Using RwLock to receive values with match
    tokio::spawn(async move {
        match rx_clone.read().await.recv().await {
            Some(value) => {
                println!("[recv match] Received: {}", value);
            }
            None => {
                println!("[recv match] Received: None");
            }
        }

        match rx_clone.read().await.recv().await {
            Some(value) => {
                println!("[recv match] Received: {}", value);
            }
            None => {
                println!("[recv match] Received: None");
            }
        }
    }).await.unwrap();

    // Using RwLock to receive values with while let
    tokio::spawn(async move {
        while let Some(value) = rx.write().await.recv().await {
            println!("[recv while let] Received: {}", value);
        }
        println!("[recv while let] Channel closed");
    }).await.unwrap();
}
*/




// tokio::join! or futures::join! usage: (quite similar to the previous verison await.unwrap())
/*
extern crate futures;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{Duration, timeout};
use futures::future::try_join;

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // Receiver must be mutable in order to call ".recv()"
    let (tx, rx) = mpsc::channel::<i32>(10);

    // (By wrapping the original receiver `rx` in an Arc and a Mutex, `rx` does not need to be mutable anymore)
    // * Explain: The Arc allows you to share ownership of `rx` across multiple tasks,
    // * and the Mutex ensures that only one task at a time can have mutable access to `rx`.
    // * After this line, `rx` is no longer mutable, and you can safely share it among asynchronous tasks.
    // Note: `tokio::sync::Mutex` or `futures::lock::Mutex` both work
    let rx = std::sync::Arc::new(Mutex::new(rx));
    let rx_clone = std::sync::Arc::clone(&rx);

    // Using Sink trait to send values
    let send_task = tokio::spawn(async move {
        // tokio::time::sleep(Duration::from_micros(10)).await;
        tx.send(42).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        tx.send(99).await.unwrap();
    });

    // Using Mutex lock to receive values with recv()
    let recv_match_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        match rx_clone.lock().await.recv().await {
            Some(value) => {
                println!("[recv match] Received: {}", value);
            }
            None => {
                println!("[recv match] Received: None");
            }
        }
    });

    // Using Mutex lock to receive values with recv()
    let recv_while_let_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // If the recv() returns None, indicating that the channel is closed, the while let loop will terminate.
        while let Some(value) = rx.lock().await.recv().await {
            println!("[recv while let] Received: {}", value);
        }
        println!("[recv while let] Channel closed"); // without this, it just skips
    });

    // Use tokio::time::timeout to set a maximum duration for the execution
    // let timeout_duration = Duration::from_secs(5); // Adjust the duration as needed
    // match timeout(timeout_duration, futures::try_join!(send_task, recv_match_task, recv_while_let_task)) {
    //     Ok(_) => println!("All tasks completed within the timeout."),
    //     Err(_) => println!("Timeout reached. Some tasks may not have completed."),
    // }


    // Waits on 3 concurrent branches, returning when all branches complete.
    // (either `futures::join!` or `tokio::join!`)
    let (first, second, third) = tokio::join!(send_task, recv_match_task, recv_while_let_task);

    // tokio::join!(send_task);

    // futures_util::future::select(recv_match_task, recv_while_let_task).await;


    // similar to futures_util::future::select()
    // tokio::select! {
    //     _ = recv_match_task => {},
    //     _ = recv_while_let_task => {}
    // };



    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    let _ = tokio::signal::ctrl_c().await;
    println!("Ctrl-c received! Terminating...");
}
*/





/*
// Returning a Result inside a Future.
// This is a common pattern when dealing with asynchronous code that may
// involve synchronous operations that can return errors.
#[allow(unused)]
use std::io;
use tokio::time::{sleep, Duration};


// The async_operation function returns a Future that yields a Result<String, io::Error>.
// It involves a call to synchronous_operation, which is a synchronous function returning a Result.
async fn async_operation() -> Result<String, io::Error> {
    // Asynchronous or synchronous computation
    let result = synchronous_operation()?;
    // Simulating an asynchronous delay
    // sleep(Duration::from_millis(10)).await;
    Ok(result)
}

fn synchronous_operation() -> Result<String, io::Error> {
    // Synchronous computation that can return a Result
    Ok("Hello, World!".to_string())
}

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    match async_operation().await {
        Ok(result) => println!("Async Operation Result: {}", result),
        Err(err) => eprintln!("Async Operation Error: {}", err),
    }

    // if dont use match, just unwrap() it to aquire the value out of Result
    let res: String = async_operation().await.unwrap();
    println!("Async Operation Result: {}", res);


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/





/*
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    value: i32,
}

impl std::future::Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling MyFuture");

        // Simulating some asynchronous work
        // In a real-world scenario, you might perform I/O, computation, or other async operations here
        // For simplicity, we'll just return Poll::Pending for the first call and Poll::Ready after that

        if self.value == 0 {
            // Simulate asynchronous work not completed yet
            self.get_mut().value += 1;
            Poll::Pending
        } else {
            // Simulate asynchronous work completed
            Poll::Ready(self.get_mut().value)
        }
    }
}

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let mut my_future: MyFuture = MyFuture { value: 0 };

    // Poll the future using `poll` method
    let mut my_future: Pin<&mut MyFuture> = Pin::new(&mut my_future);

    // First poll (should print "Polling MyFuture")
    match my_future.as_mut().poll(&mut Context::from_waker(futures::task::noop_waker_ref())) {
        Poll::Pending => println!("First poll: Pending"),
        Poll::Ready(val) => println!("First poll: Ready({})", val),
    }

    // Second poll (should print "Polling MyFuture" and "Second poll: Ready(1)")
    match my_future.as_mut().poll(&mut Context::from_waker(futures::task::noop_waker_ref())) {
        Poll::Pending => println!("Second poll: Pending"),
        Poll::Ready(val) => println!("Second poll: Ready({})", val),
    }


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
use futures::future::FutureExt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

// Our custom future type
struct MyFuture {
    value: i32,
}

impl futures::Future for MyFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling MyFuture");

        // Simulate some asynchronous work
        thread::sleep(Duration::from_millis(100));

        // In a real scenario, you would perform the asynchronous work here.
        // For simplicity, we'll just return a ready value.
        self.value += 1;

        println!("Current value: {}", self.value);

        // Assume the future is always ready for simplicity
        Poll::Ready(self.value)
    }
}

// A function that takes a Send trait bound Future
fn run_future<F>(future: F) -> i32
where
    F: futures::Future<Output = i32> + Send + 'static,
{
    // Spawn the future on a new thread
    let handle = std::thread::spawn(|| {
        // Run the future and get the result
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { future.await })
    });

    // Wait for the thread to finish and get the result
    handle.join().unwrap()
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // Create an instance of our custom future
    let my_future = MyFuture { value: 0 };

    // Run the future using the run_future function
    let result = run_future(my_future);

    println!("Final result: {}", result);

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
// problem: https://stackoverflow.com/questions/58173711/how-can-i-store-an-async-function-in-a-struct-and-call-it-from-a-struct-instance?rq=3
async fn foo(x: u8) -> u8 {
    2 * x
}

struct S {
    foo: (),
}

async fn example() {
    let s = S { foo };
}
*/



/* 
// solution:
async fn foo(x: u8) -> u8 {
    2 * x
}

struct S<F>
where
    F: std::future::Future,
{
    foo: fn(u8) -> F,
}

impl<F> S<F>
where
    F: std::future::Future,
{
    async fn do_thing(self) {
        (self.foo)(42).await;
    }
}

async fn example() -> u8 {
    let s = S { foo };
    s.do_thing().await;

    // try returning an u8 value (instead of () )
    1
}

// use std::error::Error;
use std::future::IntoFuture;

#[tokio::main]
async fn main() {
    let res = example().into_future();
    println!("{:?}", res.await);

    // Ok(())

}
*/


// Conversion into a Future
// By implementing IntoFuture for a type, you define how it will be converted to a future.
// use std::future::IntoFuture;

// #[tokio::main]
// async fn main() {
//     let v = async { "meow" };
//     let mut fut = v.into_future();
//     // assert_eq!("meow", fut.await);
//     println!("{}", fut.await);
// }






/*
// https://doc.rust-lang.org/std/future/trait.IntoFuture.html

use std::future::{ready, Ready, IntoFuture};

/// Eventually multiply two numbers
pub struct Multiply {
    num: u16,
    factor: u16,
}

impl Multiply {
    /// Construct a new instance of `Multiply`.
    pub fn new(num: u16, factor: u16) -> Self {
        Self { num, factor }
    }

    /// Set the number to multiply by the factor.
    pub fn number(mut self, num: u16) -> Self {
        self.num = num;
        self
    }

    /// Set the factor to multiply the number with.
    pub fn factor(mut self, factor: u16) -> Self {
        self.factor = factor;
        self
    }
}

impl IntoFuture for Multiply {
    type Output = u16;
    type IntoFuture = Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        ready(self.num * self.factor)
    }
}

// NOTE: Rust does not yet have an `async fn main` function, that functionality
// currently only exists in the ecosystem.
async fn run() {
    let num = Multiply::new(0, 0)  // initialize the builder to number: 0, factor: 0
        .number(2)                 // change the number to 2
        .factor(2)                 // change the factor to 2
        .await;                    // convert to future and .await

    assert_eq!(num, 4);
}

#[tokio::main]
async fn main() {
    run().await;
    println!("it worked");
}
*/




/*
use std::sync::mpsc;
use std::thread;
use tokio::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}
*/




/* 
//  (partially documented, research still in progress)
use tokio::sync::mpsc;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    tokio::spawn(async move {
        let thread_id: std::thread::ThreadId = std::thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {}", i)).await.unwrap();
            println!("{thread_id:?}: sent Message {i}");
            let _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        println!("{thread_id:?}: done");
    });

    // Sleep for a short duration to allow the spawned thread to send messages
    // tokio::time::sleep(Duration::from_millis(100)).await;

    // while let Some(msg) = rx.recv().await {
    //     println!("Main: got {msg}");
    // }

    tokio::spawn(async move {
        // tokio::time::sleep(Duration::from_millis(200)).await;
        while let Some(msg) = rx.recv().await {
            println!("Main: got {msg}");
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }).await.unwrap();
}
*/




/*
use tokio::sync::mpsc;
use tokio::time::Duration;
use std::thread;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // Task 1: Sending messages
    tokio::spawn(async move {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {}", i)).await.unwrap();
            println!("{thread_id:?}: sent Message {i}");
            let _ = tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
        println!("{thread_id:?}: done");
    });

    // Sleep for a short duration to allow the spawned thread to send messages
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Task 2: Receiving messages
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Main: got {msg}");
        }
    }).await.unwrap();
}
*/





/*
// abort
use tokio::time::{sleep, Duration};

async fn my_async_function() {
    // Simulate some asynchronous work
    sleep(Duration::from_secs(2)).await;
    println!("Async function completed");
}

async fn main_task() {
    // A flag to indicate whether the task should be aborted
    let mut should_abort = false;

    // The main task that runs in the background
    let background_task = tokio::spawn(async move {
        while !should_abort {
            println!("Background task running");
            sleep(Duration::from_secs(1)).await;
        }
        println!("Background task aborted");
    });

    // The main logic with a timeout
    tokio::select! {
        _ = my_async_function() => {
            println!("Main task completed");
        }
        _ = sleep(Duration::from_secs(5)) => {
            // If the timeout is reached, set the flag to abort the background task
            should_abort = true;
            println!("Timeout reached, aborting background task");
        }
    }

    // Wait for the background task to finish
    let _ = background_task.await;
}

#[tokio::main]
async fn main() {
    main_task().await;
}
*/





/*
// custom spawn() method that returns a JoinHandle (documented)
use tokio::task::JoinHandle;

// Your custom async function
async fn my_custom_async_function() {
    // Your asynchronous logic here
    println!("Custom async function completed");
}

// Custom spawn function that returns a JoinHandle
fn custom_spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: std::future::Future + Send + 'static,
    F::Output: Send,
{
    tokio::task::spawn(future)
}

#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();

    // Use the custom_spawn function to spawn your async function
    let handle: JoinHandle<()> = custom_spawn(my_custom_async_function());

    // Wait for the spawned task to complete
    let result: () = handle.await.unwrap();
    println!("Result: {:?}", result);
    
    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/




/*
// spawning concurrent tokio tasks
use tokio::time::{sleep, Duration};
use std::time::Instant;
use tokio::task;

async fn my_custom_async_function(value: u8) -> Result<u8, &'static str> {
    // Simulate some asynchronous computation
    sleep(Duration::from_millis(10)).await;

    // Return a Result with the modified value
    Ok(value + 1)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_time: Instant = Instant::now();

    // Create two instances of the custom asynchronous function
    let task1: task::JoinHandle<Result<u8, &str>> = tokio::spawn(my_custom_async_function(1));
    let task2: task::JoinHandle<Result<u8, &str>> = tokio::spawn(my_custom_async_function(2));

    // Await the results of both tasks
    let result1: Result<u8, &str> = task1.await?;
    let result2: Result<u8, &str> = task2.await.unwrap();

    // Combine the results
    let combined_result = result1? + result2.unwrap();

    println!("Combined Result: {}", combined_result);

    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    Ok(())
}
*/




/*
extern crate chrono;
use chrono::Local;
use std::thread;
use tokio::{self, task, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    // let rt = Runtime::new().unwrap();
    let x = 3;
    // let _guard = rt.enter();
    tokio::spawn(async {
        tokio::time::sleep(time::Duration::from_millis(200)).await;
        println!("task over: {}", now());
    }).await;

    let task2 = tokio::spawn(async move{x*x});
    
    // drop(_guard);
    println!("main thread print");
    tokio::time::sleep(time::Duration::from_millis(100)).await;
    tokio::spawn(async move {
        let result = task2.await.unwrap();
        println!("result={}",result);
        
    }).await;
}

// 作者：shelgi
// 链接：https://juejin.cn/post/7262372637283319866
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/


/*
#[tokio::main]
async fn main() {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));

    let (tx, mut rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        tx.send("aaa").unwrap();
      });
    loop {
        // 注意，select!中无需await，因为select!会自动轮询推进每一个分支的任务进度
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut rx => {
                println!("Got message: {}", msg.unwrap());
                break;
            }
        }
    }
}

// 作者：shelgi
// 链接：https://juejin.cn/post/7262372637283319866
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/



/*
// use async_std::task::sleep;
use tokio::time::sleep;
use futures;

async fn task1()->i32{
    println!("task1....");
    return 1;
}

async fn task2(x:i32){
    let y=x+1;
    println!("task2 calculate finish x+1= {}",y);
}

async fn task(){
    let input=task1().await;
    println!("preparing input...");
    sleep(tokio::time::Duration::from_millis(100)).await;
    task2(input).await;
}

async fn task3(){
    println!("task3...");
}

async fn async_main(){
    let f1= task();
    let f2=task3();
    futures::join!(f1,f2);
}

#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();


    futures::executor::block_on(async_main());

    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

   
//    作者：shelgi
//    链接：https://juejin.cn/post/7260512655701360696
//    来源：稀土掘金
//    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/




/* 
// documented
use std::task::Context;
use std::task::Poll;

// use futures::*;
use futures::StreamExt;  // in order to use stream.next()

use futures_util::Stream;

#[derive(Clone, Copy)]
struct Counter{
    count:usize,
}

impl Counter{
    fn new()->Counter{
        Counter{count:0}
    }
}

impl Stream for Counter {
    type Item = usize;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.count+=1;
        if self.count<10{
            std::task::Poll::Ready(Some(self.count))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

async fn sum1(stream: impl Stream<Item=usize>) {
    futures::pin_mut!(stream);
    let mut sum:usize=0;
    while let Some(item)=stream.next().await{
        sum+=item;
    }
    println!("sum1={}",sum);
}

async fn sum2(stream: impl Stream<Item=usize>) {
    futures::pin_mut!(stream);
    let mut sum: usize = 0;
    loop{
        match stream.next().await {
            Some(item) => {sum+=item}
            None => break
      }
    }
    println!("sum2={}", sum);
}
async fn async_main(){
    let count_stream=Counter::new();
    let stream2 = futures::stream::iter(1..20);
    let f1= sum1(count_stream);
    let f2=sum2(stream2);
    futures::join!(f1,f2);
}
   

// async fn async_main(){
//     let count_stream: Counter = Counter::new();

//     let f1 = sum1(count_stream);
//     let f2= sum2(count_stream);
//     futures::join!(f1,f2);
// }

#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();


    futures::executor::block_on(async_main());


    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

// 作者：shelgi
// 链接：https://juejin.cn/post/7260512655701360696
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

*/



/* 
// documented
// use futures::StreamExt;
use futures_util::FutureExt;

async fn task1(){
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("task1...");
}
async fn task2(){
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("task2...");
}

async fn select_task(){
    let t1 = task1().fuse();    //Fuse<impl Future<Output = ...>>
    let t2 = task2().fuse();
    tokio::pin!(t1, t2);
    futures_util::select! {
        ()= t1 =>println!("task1 finish!"),
        ()= t2 =>println!("task2 finish!"),
    }
}

#[tokio::main]
async fn main(){
    let main_time = tokio::time::Instant::now();
    futures::executor::block_on(select_task());
    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
// simple tcp server (documented)
async fn handle_connection(mut stream: tokio::net::TcpStream) {
    // 从连接中顺序读取 1024 字节数据
    let mut buffer = [0; 1024];
    tokio::io::AsyncReadExt::read(&mut stream, &mut buffer).await.unwrap();

    // let get = b"GET / HTTP/1.1\r\n";
    // 处理HTTP协议头，若不符合则返回404和对应的 `html` 文件
    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "src/hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/404.html")
    // };
    // let contents = tokio::fs::read_to_string(filename).await.unwrap();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    let contents = "hello".to_string();

    // 将回复内容写入连接缓存中
    let response = format!("{status_line}{contents}");
    tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await.unwrap();
    // 使用 flush 将缓存中的内容发送到客户端
    tokio::io::AsyncWriteExt::flush(&mut stream).await.unwrap();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    /* 
    // async_std
    let listener = async_std::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener.incoming().for_each_concurrent(10, |stream| async move {
    let stream = stream.unwrap();
        tokio::spawn(handle_connection(stream));
    }).await;
    */
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await?;
        handle_connection(socket).await;
    }
}
*/




/*
use futures;
use std::{future::Future, pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, Waker}, thread, time::Duration};

fn main() {
    // 我们现在还没有实现调度器，所以要用一下futues库里的一个调度器。
    // (We haven't implemented a scheduler yet, so we need to use a scheduler in the futures library.)
    futures::executor::block_on(TimerFuture::new(Duration::new(1, 0)));    
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

// 我们想要实现一个定时器Future (We want to implement a timer Future)
pub struct TimerFuture {
    share_state: Arc<Mutex<SharedState>>,
}

// impl Future trait for TimerFuture.
impl Future for TimerFuture {
    type Output = ();
    // executor will run this poll ,and Context is to tell future how to wakeup the task.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut share_state = self.share_state.lock().unwrap();
        if share_state.completed {
            println!("future ready. execute poll to return.");
            Poll::Ready(())
        } else {
            println!("future not ready, tell the future task how to wakeup to executor");
            // 你要告诉future，当事件就绪后怎么唤醒任务去调度执行，而这个waker根具体的调度器有关
            // (You have to tell the future how to wake up the task to schedule execution when the event is ready, and this waker is related to the specific scheduler.)
            // 调度器执行的时候会将上下文信息传进来，里面最重要的一项就是Waker
            // (When the scheduler is executed, the context information will be passed in. The most important one is Waker.)
            share_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let share_state = Arc::new(Mutex::new(SharedState{completed:false, waker:None}));
        let thread_shared_state = share_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut share_state = thread_shared_state.lock().unwrap();
            share_state.completed = true;
            if let Some(waker) = share_state.waker.take() {
                println!("detect future is ready, wakeup the future task to executor.");
                waker.wake()    // wakeup the future task to executor.
            }
        });

        TimerFuture {share_state}
    }
}
// 作者：chirpyli
// 链接：https://juejin.cn/post/6844903973950849031
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/



/*
use std::future::IntoFuture;
use futures::*;

/// Convert the output of a future to a string.
async fn fut_to_string<Fut>(fut: Fut) -> String
where
    Fut: IntoFuture,
    Fut::Output: std::fmt::Debug,
{
    format!("{:?}", fut.await)
}

async fn a_future() {
    println!("a future");
}

#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();
    
    let fut = a_future().into_future();

    // futures::executor::block_on(fut_to_string(fut));

    let res: String = async { fut_to_string(fut).await }.await;

    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
// traits & dyn
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// 错误示例：无法为trait实现动态大小类型
// impl Shape for dyn Shape {
//     fn area(&self) -> f64 {
//         // 实现trait方法
//     }
// }

fn main() {
    let circle: Circle = Circle { radius: 5.0 };
    let shape: &dyn Shape = &circle;
    println!("{}", shape.area());

    println!("{}", &circle.area());
}

// 作者：繁依Fanyi
// 链接：https://juejin.cn/post/7263744906681696316
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/



/*
use futures::executor::block_on;
use std::time::Duration;
use tokio::time;

async fn async_function() {
    // time::sleep(Duration::from_millis(10)).await;
    println!("async_function completed");
}

async fn another_async_function() {
    // time::sleep(Duration::from_secs(2)).await;
    println!("another_async_function completed");
}

async fn multiple_async_functions() {
    let f1 = async_function();
    let f2 = another_async_function();
    futures::join!(f1, f2);
}

#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();

    let future = multiple_async_functions();
    block_on(future);

    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
// pinning (researching, not documented)
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
struct MyStruct {
    data: i32,
}

impl MyStruct {
    fn new(data: i32) -> Self {
        MyStruct { data }
    }

    fn print_data(self: Pin<&Self>) {
        println!("Memory address: {:p}", self);
        println!("Data: {}", self.data);
    }
}

async fn update_data(data: Arc<Mutex<MyStruct>>, new_value: i32) {
    // Simulate some asynchronous work
    sleep(Duration::from_millis(100)).await;

    // Lock the mutex to get mutable access to MyStruct
    let mut data_mut = data.lock().unwrap();

    // Update the data value
    data_mut.data = new_value;
}

#[tokio::main]
async fn main() {
    // Create a shared instance of MyStruct using Arc and Mutex
    let data: Arc<Mutex<MyStruct>> = Arc::new(Mutex::new(MyStruct::new(42)));

    // Convert the Arc<Mutex<MyStruct>> to a pinned reference
    let binding: std::sync::MutexGuard<'_, MyStruct> = data.lock().unwrap();
    let pinned_data: Pin<&MyStruct> = Pin::new(&binding);

    // Print the initial data using the pinned reference
    pinned_data.print_data();

    // Clone the Arc<Mutex<MyStruct>> to be able to share it across tasks
    let data_clone: Arc<Mutex<MyStruct>> = data.clone();

    // Release the lock to allow other tasks to acquire it
    drop(binding);

    // Spawn multiple asynchronous tasks to update the data concurrently
    let task1: tokio::task::JoinHandle<()> = tokio::spawn(update_data(data.clone(), 99));
    let task2: tokio::task::JoinHandle<()> = tokio::spawn(update_data(data.clone(), 123));

    // Wait for tasks to complete
    tokio::try_join!(task1, task2).unwrap();

    // Acquire the lock again before printing the final data
    let binding: std::sync::MutexGuard<'_, MyStruct> = data.lock().unwrap();
    let pinned_data: Pin<&MyStruct> = Pin::new(&binding);

    // Print the final data using the pinned reference
    pinned_data.print_data();
}
*/




/*
use std::{future::Future, pin::Pin, task::{Context, Poll}, time::{Duration, Instant}};
use pin_project::pin_project;   // crate: pin-project = "1.1.3"

/// A future which returns a random number when it resolves.
#[derive(Default)]
struct RandFuture;

impl Future for RandFuture {
    // Every future has to specify what type of value it returns when it resolves.
    // This particular future will return a u16.
    type Output = u16;

    // The `Future` trait has only one method, named "poll".
    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(rand::random())
    }
}
#[pin_project] // This generates a `project` method
pub struct TimedWrapper<Fut: Future> {
    // For each field, we need to choose whether `project` returns an
    // unpinned (&mut T) or pinned (Pin<&mut T>) reference to the field.
    // By default, it assumes unpinned:
    start: Option<Instant>,
    // Opt into pinned references with this attribute:
    #[pin]
    future: Fut,
}
impl<Fut: Future> Future for TimedWrapper<Fut> {
    // This future will output a pair of values:
    // 1. The value from the inner future
    // 2. How long it took for the inner future to resolve
    type Output = (Fut::Output, Duration);

    // fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
    //     // Call the inner poll, measuring how long it took.
    //     let start = self.start.get_or_insert_with(Instant::now);
    //     let inner_poll = self.future.poll(cx);
    //     let elapsed = self.elapsed();
     
    //     match inner_poll {
    //         // The inner future needs more time, so this future needs more time too
    //         Poll::Pending => Poll::Pending,
    //         // Success!
    //         Poll::Ready(output) => Poll::Ready((output, elapsed)),
    //     }
    // }
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // This returns a type with all the same fields, with all the same types,
        // except that the fields defined with #[pin] will be pinned.
        let mut this = self.project();
        
        // Call the inner poll, measuring how long it took.
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.as_mut().poll(cx);
        let elapsed = start.elapsed();
        
        match inner_poll {
            // The inner future needs more time, so this future needs more time too
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}
   

#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();

    // println!("lon ba");

    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

// 作者：RustMan
// 链接：https://juejin.cn/post/7001844112659857421
// Original source: https://blog.adamchalmers.com/pin-unpin/
// Post title: 为什么 Rust 需要 Pin, Unpin ？
*/





// // (not documented)
// // https://www.rectcircle.cn/posts/rust%E5%BC%82%E6%AD%A5%E7%BC%96%E7%A8%8B/
// use std::future::Future;
// use std::pin::Pin;
// use std::task::{Waker, Poll, Context};
// use futures::task::{ArcWake, waker_ref};
// use std::sync::{Mutex, Arc};
// use futures::future::FutureObj;
// use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

// // === 实现一个Future (Implement a Future) ===
// // 实现一个异步的sleep类似于async_std::task:sleep
// // 实现原理 创建一个新的线程，然后立即sleep，唤醒后，更改状态，唤醒轮训
// // (Implementation principle: Create a new thread, then sleep immediately. After waking up, change the state and wake up for rotation training.)

// pub struct TimerFuture {
//         shared_state: Arc<Mutex<SharedState>>,
//     }
    
//     /// `future`与等待线程之间的共享状态 (Shared state between `future` and waiting threads)
//     struct SharedState {
//         /// 用于判断sleep的时间是不是已经过了
//         completed: bool,
        
//         /// The waker for the task that `TimerFuture` is running on.
//         /// The thread can use this after setting `completed = true` to tell
//         /// `TimerFuture`'s task to wake up, see that `completed = true`, and
//         /// move forward.
//         waker: Option<Waker>,
//     }
    
//     impl Future for TimerFuture {
//         type Output = ();

//         // 轮询函数 (Polling function)
//         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//             // 观察睡眠时间是否完成 (Observe whether the sleep time is completed)
//             println!("###sleep poll");

//             // Look at the shared state to see if the timer has already completed.
//             let mut shared_state = self.shared_state.lock().unwrap();
//             if shared_state.completed {

//                 // 已经完成，就绪 (Completed, ready)
//                 Poll::Ready(())
//             } else {
//                 // Set waker so that the thread can wake up the current task
//                 // when the timer has completed, ensuring that the future is polled
//                 // again and sees that `completed = true`.
//                 //
//                 // It's tempting to do this once rather than repeatedly cloning
//                 // the waker each time. However, the `TimerFuture` can move between
//                 // tasks on the executor, which could cause a stale waker pointing
//                 // to the wrong task, preventing `TimerFuture` from waking up
//                 // correctly.
//                 //
//                 // N.B. it's possible to check for this using the `Waker::will_wake`
//                 // function, but we omit that here to keep things simple.
//                 shared_state.waker = Some(cx.waker().clone()); // 相当于下方Task引用计数+1，导致Task不被销毁，从而导致SyncSender引用计数大于零，循环得以继续
//                 Poll::Pending
//             }
//         }
//     }
    
//     impl TimerFuture {
//         /// Create a new `TimerFuture` which will complete after the provided
//         /// timeout.
//         /// 构造函数 (Constructor)
//         pub fn new(duration: std::time::Duration) -> Self {
//             // 构造共享状态 (Construct shared state)
//             let shared_state: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState {
//                 completed: false,
//                 waker: None,
//             }));
            
//             // 创建新的线程 (Create new thread)
//             let thread_shared_state: Arc<Mutex<SharedState>> = shared_state.clone();
//             std::thread::spawn(move || {
//                 // 睡眠 (sleep)
//                 std::thread::sleep(duration);
//                 // 获得锁 (get lock)
//                 let mut shared_state: std::sync::MutexGuard<'_, SharedState> = thread_shared_state.lock().unwrap();
//                 // 标记为已完成 (Mark as completed)
//                 shared_state.completed = true;
//                 // 唤醒轮询 (wake polling)
//                 if let Some(waker) = shared_state.waker.take() {
//                     waker.wake() // 将任务重新放到任务队列 (Put the task back into the task queue)
//                 }
//             });
            
//             TimerFuture { shared_state }
//         }
//     }
    
//     // use futures::executor::block_on;
    
//     // {
//     // println!("===测试自定义Future===");
//     // block_on(async {
//     //     println!("睡眠前!");
//     //     // Wait for our timer future to complete after two seconds.
//     //     TimerFuture::new(time::Duration::from_secs(2)).await;
//     //     // for i in 0..2 {
//     //     //     TimerFuture::new(time::Duration::from_secs(i)).await;
//     //     // }
//     //     println!("睡眠后!");
//     // });
//     // }

// // next section: Applied: Build an Executor: https://rust-lang.github.io/async-book/02_execution/04_executor.html
// // === 实现一个Future执行器 (Implement a Future executor) ===

// /// Task executor that receives tasks off of a channel and runs them.
// struct Executor {
//     ready_queue: Receiver<Arc<Task>>, // 任务接收端 (Task receiver)
// }

// /// 用于向执行器中推送任务 (Used to push tasks to the executor)
// /// `Spawner` spawns new futures onto the task channel.
// #[derive(Clone)]
// struct Spawner {
//     task_sender: SyncSender<Arc<Task>>, // 任务发送端 (Task sender)
// }

// /// A future that can reschedule itself to be polled by an `Executor`.
// struct Task {
//     // In-progress future that should be pushed to completion
//     //
//     // The `Mutex` is not necessary for correctness, since we only have
//     // one thread executing tasks at once. However, `rustc` isn't smart
//     // enough to know that `future` is only mutated from one thread,
//     // so we use it in order to provide safety. A production executor would
//     // not need this, and could use `UnsafeCell` instead.
//     // FutureObj object
//     future: Mutex<Option<FutureObj<'static, ()>>>,

//     // Handle to spawn tasks onto the task queue
//     // 任务发送者，用于将任务放到任务队列 (Task sender, used to put tasks into the task queue)
//     task_sender: SyncSender<Arc<Task>>,
// }

// impl Spawner {
//     /// 将任务推送到队列 (Push task to queue)
//     fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
//         // 传递一个future，创建一个Task，并推送到任务队列
//         // (Pass a future, create a Task, and push it to the task queue)
//         let future_obj = FutureObj::new(Box::new(future));
//         let task = Arc::new(Task {
//             future: Mutex::new(Some(future_obj)),
//             task_sender: self.task_sender.clone(),
//         });
//         self.task_sender.send(task).expect("too many tasks queued");
//     }
// }


// impl ArcWake for Task {
//     fn wake_by_ref(arc_self: &Arc<Self>) {
//         // Implement `wake` by sending this task back onto the task channel
//         // so that it will be polled again by the executor.
//         let cloned = arc_self.clone();
//         arc_self.task_sender.send(cloned).expect("too many tasks queued");
//     }
// }

// impl Executor {
//     /* modified chinese code
//     fn run(&self) {
//         while let Ok(task) = self.ready_queue.recv() {
//             // Take the future, and if it has not yet completed (is still Some),
//             // poll it in an attempt to complete it.
//             // 从任务中拿出future，如果没有完成，将返回some
//             let mut future_slot = task.future.lock().unwrap();
//             println!("+++[exec] task.strong_count = {:?}", Arc::strong_count(&task));
//             if let Some(mut future) = future_slot.take() { // 在此take出来了 (Take it here)
//                 // Create a `LocalWaker` from the task itself
//                 // which actually calls Task's wake_by_ref
//                 let waker = waker_ref(&task); // 给Task (Give to Task)
//                 let context = &mut Context::from_waker(&*waker);
//                 if let Poll::Pending = Pin::new(&mut future).poll(context) {
//                     // If it is not completed, put it back into the lock object and wait for wake to execute again.
//                     println!("---[exec] task.strong_count = {:?}", Arc::strong_count(&task));
//                     *future_slot = Some(future); // Needs to be put back in the lock
//                 }
//             }
//         }
//     }
//     */

//     //* original code from post
//     fn run(&self) {
//         while let Ok(task) = self.ready_queue.recv() {
//             // Take the future, and if it has not yet completed (is still Some),
//             // poll it in an attempt to complete it.
//             let mut future_slot = task.future.lock().unwrap();
//             if let Some(mut future) = future_slot.take() {
//                 // Create a `LocalWaker` from the task itself
//                 let waker = waker_ref(&task);
//                 let context = &mut Context::from_waker(&waker);
//                 // `BoxFuture<T>` is a type alias for
//                 // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
//                 // We can get a `Pin<&mut dyn Future + Send + 'static>`
//                 // from it by calling the `Pin::as_mut` method.

//                 // the `.poll` method only allow `Pin<&mut FutureObj<'_, ()>>` !!

//                 // `alloc::boxed::Box::pin` takes ownership of the pinnned value/future.
//                 // if let Poll::Pending = Box::pin(future).as_mut().poll(context) {
//                 //     // We're not done processing the future, so put it
//                 //     // back in its task to be run again in the future.
//                 //     *future_slot = Some(future);     -> error: use of moved value `future`
//                 // }
                
//                 // So we use `core::pin::Pin::new`, which accept a mutable reference.
//                 if let Poll::Pending = Pin::new(&mut future).poll(context) {
//                     // If it is not completed, put it back into the lock object and wait for wake to execute again.
//                     println!("---[exec] task.strong_count = {:?}", Arc::strong_count(&task));
//                     *future_slot = Some(future); // Needs to be put back in the lock
//                 }
//             }
//         }
//     }
//     //*/
// }

// /// Create executors and task pushers
// fn new_executor_and_spawner() -> (Executor, Spawner) {
//     // Maximum number of tasks to allow queueing in the channel at once.
//     // This is just to make `sync_channel` happy, and wouldn't be present in
//     // a real executor.

//     // Message queue maximum size
//     const MAX_QUEUED_TASKS: usize = 10_000;
//     let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
//     (Executor { ready_queue }, Spawner { task_sender})
// }


// fn main() {
//     println!("===测试自定义Executor (Test custom Executor)===");
//     let (executor, spawner) = new_executor_and_spawner();
//     spawner.spawn(async {
//         println!("howdy!");
//         // Wait for our timer future to complete after two seconds.
//         TimerFuture::new(std::time::Duration::from_millis(200)).await;
//         // for i in 0..2 {
//         //     TimerFuture::new(time::Duration::from_secs(i)).await;
//         // }
//         println!("done!");
//     });
//     // Drop the spawner so that our executor knows it is finished and won't
//     // receive more incoming tasks to run.
//     // 销毁发送端以可以结束 (Destroy the sender to end)
//     drop(spawner);
//     // 如何实现所有任务执行完毕，自动返回的呢？
//     // 本质上是生命周期和Arc引用计数（Task持有一个SyncSender）
//     // Task实现了ArcWake特质，std::sync::Arc<Task> 本质上 就是Future::poll函数Context中weak的实现
//     // Future当处于等待的时候，weak.clone，让std::sync::Arc<Task>引用计数保持大于1，轮训继续进行
//     // 当Future完成后，std::sync::Arc<Task>引用计数等于0，Task被销毁（在本例中就是睡眠线程执行完成，并销毁）
//     // 从而SyncSender被销毁，
//     // 当所有Task被销毁，SyncSender真正被销毁，导致Receiver返回Err
//     // 从而run返回
//     executor.run();
// }






/*
fn bar() -> impl std::future::Future {
    std::future::from_generator(move |mut _task_context| {
        let _t = {
            match std::future::IntoFuture::into_future(foo()) {
                mut __awaitee => loop {
                    match unsafe {
                        std::future::Future::poll(
                            std::pin::Pin::new_unchecked(&mut __awaitee),
                            std::future::get_context(_task_context),
                        )
                    } {
                        std::task::Poll::Ready { 0: result } => break result,
                        std::task::Poll::Pending {} => {}
                    }
                    _task_context = (yield ());
                },
            };
        };
        _t
    })
}
*/




/*
// async with backtrace: https://swatinem.de/blog/async-codegen/
use std::backtrace::Backtrace;

pub async fn a(arg: u32) -> Backtrace {
    let bt = b().await;
    let _arg = arg;
    bt
}

pub async fn b() -> Backtrace {
    Backtrace::force_capture()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stack() {
        let backtrace = a(0).await;
        println!("{}", backtrace);
    }
}

#[tokio::main]
async fn main() {
    let backtrace: Backtrace = a(0).await;
    println!("{}", backtrace);
}
*/



// create a slice pointer when starting out with a pointer to the first element
// #[cfg(test)]
// mod tests {
//     // use super::*;    // this imports every fucking thing
//     use std::ptr;

//     #[tokio::test]
//     async fn it_works() {
//         let x: [i32; 3] = [5, 6, 7];
//         let raw_pointer: *const i32 = x.as_ptr();
//         println!("{:p}", &raw_pointer);
//         let slice: *const [i32] = ptr::slice_from_raw_parts(raw_pointer, 3);
//         assert_eq!(unsafe { &*slice }[2], 7);
//     }
// }

// use std::ptr;
// #[tokio::main]
// async fn main() {
//     let x: [i32; 3] = [5, 6, 7];
//     let raw_pointer: *const i32 = x.as_ptr();
//     unsafe {
//         println!("{:?}", *raw_pointer );    // 5
//     }
//     let slice: *const [i32] = ptr::slice_from_raw_parts(raw_pointer, 3);
//     unsafe {
//         println!("ptr: {:?}, val: {:?}", slice, &*slice);
//     }
// }




// use std::thread;

// async fn self_ref() {
//     let mut v = [1, 2, 3];

//     let x = &mut v[0];
//     println!("x: {}", x);

//     // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//     thread::sleep(std::time::Duration::from_millis(50));

//     *x = 42;
//     println!("x: {}", x);
// }
// fn main() {
//     let fut = self_ref();
//     // let _ = futures::executor::block_on(async { fut.await });
//     futures::executor::block_on(&fut);
// }






// use futures::Future;
// use std::thread;

// async fn self_ref() {
//     let mut v = [1, 2, 3];

//     let x = &mut v[0];
//     println!("x: {}", x);

//     // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//     thread::sleep(std::time::Duration::from_millis(50));

//     *x = 42;
//     println!("x: {}", x);
// }

// #[tokio::main]
// async fn main() {

//     // chatgpt, error: 
//     /* error[E0277]: `dyn futures::Future<Output = ()>` cannot be unpinned
//     --> learning/src/main.rs:3975:33
//      |
// 3975 |     futures::executor::block_on(fut);
//      |     --------------------------- ^^^ the trait `Unpin` is not implemented for `dyn futures::Future<Output = ()>`
//      |     |
//      |     required by a bound introduced by this call
//      |
//      = note: consider using the `pin!` macro
//              consider using `Box::pin` if you need to access the pinned value outside of the current scope
//      = note: required for `Box<dyn futures::Future<Output = ()>>` to implement `futures::Future`
// note: required by a bound in `block_on`
//     --> /Users/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-executor-0.3.28/src/local_pool.rs:315:20
//      |
// 315  | pub fn block_on<F: Future>(f: F) -> F::Output {
//      |                    ^^^^^^ required by this bound in `block_on`

// error[E0277]: `dyn futures::Future<Output = ()>` cannot be unpinned
//     --> learning/src/main.rs:3975:5
//      |
// 3975 |     futures::executor::block_on(fut);
//      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `dyn futures::Future<Output = ()>`
//      |
//      = note: consider using the `pin!` macro
//              consider using `Box::pin` if you need to access the pinned value outside of the current scope
//      = note: required for `Box<dyn futures::Future<Output = ()>>` to implement `futures::Future`

// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `learning` (bin "learning") due to 2 previous errors
// */
//     // let fut: Box<dyn Future<Output = ()>> = Box::new(self_ref());
//     // futures::executor::block_on(fut);
    

//     // chatgpt fix (worked):
//     // we make `fut` a `Pin<Box<impl Future<Output = ()>>>` first, then call `block_on(fut)`
//     let fut = Box::pin(self_ref());

//     futures::executor::block_on(fut);


//     // try pinning (still can't be unpinned error)
//     // let mut fut = self_ref();
//     // let pinned_fut = std::pin::Pin::new(&mut fut);
//     // futures::executor::block_on(async { pinned_fut.await });


//     // try tokio::pin (require tokio::main runtime context)
//     let fut = self_ref();
//     // (&fut).await;    // to use parentheses, we must take ownership of the data
//     // tokio::pin!(fut);
//     // futures::executor::block_on(fut);    // (err: `()` is not a future )


//     // let pinned_fut = Box::pin(fut);
//     // let _ = futures::executor::block_on(async { pinned_fut });
// }





// #![feature(ptr_addr_eq)] // nighty experimental
// fn main () {
    // use std::ptr;

    // let five = 5;
    // let other_five = 5;
    // let five_ref = &five;
    // let same_five_ref = &five;
    // let other_five_ref = &other_five;

    // println!("{}", five_ref == same_five_ref);
    // println!("{}", ptr::eq(five_ref, same_five_ref));

    // println!("{}", five_ref == other_five_ref);
    // println!("{}", !ptr::eq(five_ref, other_five_ref));


    // use std::pin::Pin;

    // let mut val: u8 = 5;
    // let pinned: Pin<&mut u8> = Pin::new(&mut val);

    // // Unwrap the pin to get a reference to the value
    // let r: &mut u8 = Pin::into_inner(pinned);
    // println!("{}", *r);

    // use std::ops::Deref;

    // struct DerefExample<T> {
    //     value: T
    // }

    // impl<T> Deref for DerefExample<T> {
    //     type Target = T;

    //     fn deref(&self) -> &Self::Target {
    //         &self.value
    //     }
    // }

    // let x = DerefExample { value: "hello there" };
    // println!("{}", *x);

// }



/*
// deref impl example
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use std::fmt;

trait Pointer {
    fn as_ptr(&self) -> *const ();
    fn as_mut_ptr(&mut self) -> *mut ();
}

impl<T> Pointer for T {
    fn as_ptr(&self) -> *const () {
        self as *const T as *const ()
    }

    fn as_mut_ptr(&mut self) -> *mut () {
        self as *mut T as *mut ()
    }
}

pub struct MyCustomPin<Ptr> {
    pub pointer: Ptr,
}

impl<Ptr: Pointer> MyCustomPin<Ptr> {
    pub fn new(pointer: Ptr) -> Self {
        MyCustomPin { pointer }
    }
}

impl<Ptr: Deref> Deref for MyCustomPin<Ptr> {
    type Target = Ptr::Target;

    fn deref(&self) -> &Self::Target {
        &*self.pointer
    }
}

impl<Ptr: DerefMut> DerefMut for MyCustomPin<Ptr> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.pointer
    }
}

impl<Ptr: Pointer> fmt::Pointer for MyCustomPin<Ptr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.pointer.as_ptr(), f)
    }
}

impl<Ptr: std::fmt::Display> std::fmt::Display for MyCustomPin<Ptr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.pointer, f)
    }
}

fn main() {
    let mut data: i32 = 42;
    let my_custom_pin: MyCustomPin<&mut i32> = MyCustomPin::new(&mut data);

    // Print the address of the underlying pointer inside MyCustomPin
    println!("{:p}", my_custom_pin);

    // Using Deref
    println!("Deref: {}", *my_custom_pin);
    // Print the address of the underlying pointer inside MyCustomPin
    println!("{:p}", my_custom_pin);

    // Using DerefMut
    *my_custom_pin; // Just to demonstrate mutable dereferencing
    println!("DerefMut: {}", *my_custom_pin);
}
*/




/*
// an example of using trait AsyncReadExt & AsyncWriteExt
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn read_data<R>(mut reader: R) -> Result<(), io::Error>
where
    R: AsyncReadExt + Unpin,
{
    let mut buf = vec![0; 1024];

    loop {
        match reader.read(&mut buf).await {
            Ok(0) => break, // End of file
            Ok(n) => {
                // Process the read data (here we just print it)
                println!("Read {} bytes: {:?}", n, &buf[..n]);
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

// read data from the client's TcpStream asynchronously.
async fn handle_client(mut stream: tokio::net::TcpStream) -> Result<(), io::Error> {
    let mut buf: Vec<u8> = vec![0; 1024];

    let response: String = format!("HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nHello from process1");

    loop {
        match stream.read(&mut buf).await {
            Err(e) => {
                println!("Err: {:?}", e);
                return Err(e)
            },
            // socket closed
            Ok(0) => {
                // socket closed
                // return;
            }
            Ok(n) => {
                println!("new client: {:?}", stream.peer_addr());   // new client: Ok(127.0.0.1:52062)

                // Process the read data (here we just print it)
                println!("Read {} bytes: {:?}", n, &buf[..n]);

            }
        }
        // Write the response back to the socket
        // The write_all method is defined on the AsyncWriteExt trait (tokio::io::util::async_write_ext::AsyncWriteExt)
        // to import: use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
        // if let Err(e) = stream.write_all(response.as_bytes()).await {
        //     eprintln!("failed to write to socket; err = {:?}", e);
        //     // return;
        // }

        // To shut down the stream in the write direction, 
        // you can call the shutdown() method. This will cause the other peer
        // to receive a read of length 0, indicating that no more data will be sent.
        // This only closes the stream in one direction.
        stream.shutdown().await?;   
    }
    
    Ok(())
}

async fn connect_to_our_listener() -> Result<(), Box<dyn std::error::Error + Send>> {
    // Connect to a peer
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await;

    // Write some data.
    // stream.write_all(b"hello world!").await?;

    Ok(())
}

#[tokio::main]
async fn main() {

    // Example usage with a file
    // let file_path = "example.txt";
    // let file = tokio::fs::File::open(file_path).await.expect("Error opening file");

    // if let Err(err) = read_data(file).await {
    //     eprintln!("Error reading data: {}", err);
    // }

    // Example to read from a network stream.
    // This demonstrates the use of asynchronous reading with tokio::net::TcpListener and tokio::net::TcpStream
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.expect("Error binding to address");

    println!("Server listening on 127.0.0.1:8080");

    // while let Ok((stream, _)) = listener.accept().await {
    //     tokio::spawn(handle_client(stream));
    // }

    loop {  // or else the program will terminate after client hit the endpoint
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new client: {:?}", addr);
                // without `.await`, it break the program before the `handle_client` had chance to be called
                let _ = tokio::spawn(handle_client(socket)).await;
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
        tokio::spawn(
            async {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                connect_to_our_listener().await;    // just forget `.await` and it never fucking run !
            }
            
        ).await;
    }
    

}
*/





/*
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
        }
    });

    let mut children = vec![];
    children.push(thread::spawn(move || {
        if let Ok(stream) = TcpStream::connect("127.0.0.1:7879") {
            println!("Connected to the socket!");
        } else {
            println!("Couldn't connect to server...");
        }
    }));

    // std::thread::sleep(std::time::Duration::from_secs(1));
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
*/




/*
// service using NGINX ExpressJS:  https://webchat.caresoft.vn:8090/api/getDomain/DIENTHOAIVUI/
// https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html#method.all
use futures::executor::block_on_stream;
use futures::stream::{self, StreamExt};
use futures::task::Poll;

#[tokio::main]
async fn main() {

    // eg1
    // let mut x = 0;
    // let stream = stream::poll_fn(|_| {
    //     x += 1;
    //     match x {
    //         0..=2 => Poll::Ready(Some(x)),
    //         3 => Poll::Ready(None),
    //         _ => panic!("should not happen")
    //     }
    // }).fuse();

    // let mut iter = block_on_stream(stream);
    // // assert_eq!(Some(1), iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());

    
    // eg2
    // let stream = stream::iter(1..=3);
    // let stream = stream.then(|x| async move { x + 3 });

    // println!("{:?}", stream.collect::<Vec<_>>().await);


    // eg3
    use futures::channel::mpsc;
    use futures::stream::StreamExt;
    use std::thread;

    let (tx, rx) = mpsc::unbounded();

    thread::spawn(move || {
        for i in (0..3).rev() {
            let n = i * 3;
            tx.unbounded_send(vec![n + 1, n + 2, n + 3]).unwrap();
        }
    });

    let result: Vec<i32> = rx.concat().await;

    // assert_eq!(result, vec![7, 8, 9, 4, 5, 6, 1, 2, 3]);
    println!("{:?}", result);
}
*/






/*
use std::pin::Pin;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut data = "nodejs is a cheap thing";
    println!("data: {:?}", data);

    let pinned_data = Box::pin(data);
    println!("pinned_data: {:?}", pinned_data);

    // Simulate an asynchronous delay
    time::sleep(Duration::from_millis(200)).await;

    // After the delay, modify the original data
    data = "weak devs rely on javascript";
    println!("after original data changed, pinned_data: {:?}", pinned_data);

    // transparently "unwrap" it with .get_ref(), .get_mut(), and .into_inner()
    let unpinned_data = *Pin::into_inner(pinned_data);
    println!("unpinned_data: {:?}", unpinned_data);

    println!("data: {:?}", data);
}
*/


/*
use std::pin::Pin;
use std::future::Future;
use tokio::time;

async fn some_async_function() -> &'static str {
    // Simulate an asynchronous delay
    time::sleep(tokio::time::Duration::from_secs(1)).await;

    "result after delay"
}

#[tokio::main]
async fn main() {

    println!("{:?}", some_async_function().await);
    // Create a future and pin it
    let future: Pin<Box<dyn Future<Output = &str>>> = Box::pin(some_async_function());

    // Await the result of the future
    let result = future.await;
    
    println!("Result: {:?}", result);
}
*/




/*
use std::{pin::Pin, thread};
use std::future::Future;
use tokio::time::{self, Duration};
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

async fn some_async_function() -> Result<&'static str, &'static str> {

    // Simulate an asynchronous delay
    time::sleep(Duration::from_millis(500)).await;

    // Simulate an error condition
    if false {
        // In a real-world scenario, this error would be more meaningful.
        Err("An error occurred")
    } else {
        Ok("result after delay")
    }
}

#[tokio::main]
async fn main() {
    
    // Create a future and pin it
    let mut future: Pin<Box<dyn Future<Output = Result<&'static str, &'static str>>>> =
        Box::pin(some_async_function());
    let mut future_clone = *future.into_inner();

    // Create a Waker for the Context
    let waker: Waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx: Context<'_> = Context::from_waker(&waker);

    // loop {
    //     // Poll the future to check its status
    //     match Pin::as_mut(&mut future).poll(&mut cx) {
    //         Poll::Pending => {
    //             println!("The future is still pending");
    //         }
    //         Poll::Ready(result) => {
    //             match result {
    //                 Ok(value) => println!("The future has completed with result: {:?}", value),
    //                 Err(error) => println!("The future encountered an error: {:?}", error),
    //             }

    //             // Exit the loop when the future is ready
    //             break;
    //         }
    //     }
    //     tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;   // futures do nothing unless you `.await` or poll them
    // }

    // Poll the future to check its status
    match Pin::as_mut(&mut future).poll(&mut cx) {
        Poll::Pending => {
            println!("The future is still pending");
        }
        Poll::Ready(result) => {
            match result {
                Ok(value) => println!("The future has completed with result: {:?}", value),
                Err(error) => println!("The future encountered an error: {:?}", error),
            }
        }
    }
    // Create a LocalSet to run the async block on the same thread
    let local_set = tokio::task::LocalSet::new();

    // Spawn the async block in the LocalSet
    let splocal = local_set.spawn_local(async move {
        for i in 0..5 {
            // Poll the future to check its status
            match Pin::as_mut(&mut future).poll(&mut cx) {
                Poll::Pending => {
                    println!("The future is still pending");
                }
                Poll::Ready(result) => {
                    match result {
                        Ok(value) => println!("The future has completed with result: {:?}", value),
                        Err(error) => println!("The future encountered an error: {:?}", error),
                    }
                }
            }
            time::sleep(Duration::from_millis(50)).await;
        }
    });

    // Run the LocalSet to execute the spawned tasks
    local_set.await;

    // If you want to await the result of the future, you can do it like this:
    let result: Result<&'static str, &'static str> = future.await;
    match result {
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => println!("Error: {:?}", error),
    }
}


// RawWakerVTable implementation
// This creates a raw waker for a custom waker.
// It's a part of the low-level API related to waking tasks in the Rust asynchronous runtime.
// This creates a new instance of RawWakerVTable and initializes it with the provided functions.
// The RawWakerVTable struct represents a table of function pointers used to manipulate raw wakers
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    // The first function in the table is responsible for cloning a waker. In this case, it simply returns a new raw waker with a null pointer for the data (no associated data) and a reference to the same vtable. This is a self-cloning function because it's not doing any actual cloning.
    |_| RawWaker::new(std::ptr::null(), &VTABLE),

    // The second function is the "wake" function, which is called when a waker is woken. In this case, it does nothing (`{}`).
    |_| {},

    // The third function is the "wake_by_ref" function. It's similar to "wake" but takes a reference. Again, it does nothing (`{}`).
    |_| {},

    // The fourth function is the "drop" function, called when the last reference to a waker is dropped. In this case, it does nothing (`{}`).
    |_| {},
);
*/




/*
// low level async, only use std lib (documented)
// https://stackoverflow.com/questions/56252798/how-do-i-execute-an-async-await-function-without-using-any-external-dependencies?rq=3
async fn hello() {
    println!("Hello, World!");
}

fn main() {
    drive_to_completion(hello());
}

use std::{
    future::Future,
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

fn drive_to_completion<F>(f: F) -> F::Output
where
    F: Future,
{
    let waker = my_waker();
    let mut context = Context::from_waker(&waker);

    let mut t = Box::pin(f);
    let t = t.as_mut();

    loop {
        match t.poll(&mut context) {
            Poll::Ready(v) => return v,
            Poll::Pending => panic!("This executor does not support futures that are not ready"),
        }
    }
}

type WakerData = *const ();

unsafe fn clone(_: WakerData) -> RawWaker {
    my_raw_waker()
}
unsafe fn wake(_: WakerData) {}
unsafe fn wake_by_ref(_: WakerData) {}
unsafe fn drop(_: WakerData) {}

static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

fn my_raw_waker() -> RawWaker {
    RawWaker::new(ptr::null(), &MY_VTABLE)
}

fn my_waker() -> Waker {
    unsafe { Waker::from_raw(my_raw_waker()) }
}
*/










/*
// BoxFuture example (not-documented)
// An owned dynamically typed Future for use in cases where you can’t statically type your result or need to add some indirection.
use futures::future::BoxFuture;
use futures::prelude::*;

async fn some_async_fn() -> BoxFuture<'static, String> {
    println!("running some_async_fn()");
    future::ready("test".to_owned()).boxed()
}

fn main() {
    let bar = async {
        let foo = some_async_fn();
        println!("done running some_async_fn()");
    };
    
    futures::executor::block_on(bar);
}
*/




/*
// not-documented
// Pin prevents data getting moved the same way that reference prevent aliasing: by restricting what you can do with it. In particular functions that can move a T require either the T itself or &mut T (for example Vec::push or mem::swap). Wrapping a pointer with Pin prevents you from performing these kinds of operations, instead restricting you to ones that can take a Pin<Box<T>> or Pin<&mut T>.
use std::pin::Pin;
use std::mem;

fn move_pinned_ref<T>(mut a: T, mut b: T) {
    unsafe {
        let p: Pin<&mut T> = Pin::new_unchecked(&mut a);
        // This should mean the pointee `a` can never move again.
    }
    mem::swap(&mut a, &mut b); // Potential UB down the road ⚠️
    // The address of `a` changed to `b`'s stack slot, so `a` got moved even
    // though we have previously pinned it! We have violated the pinning API contract.
}

fn main() {
    let mut val: u8 = 5;
    println!("val: {:p}", &val);

    // We can pin the value, since it doesn't care about being moved
    let pinned: Pin<&mut u8> = Pin::new(&mut val);
    println!("pinned: {:p}", &pinned);

    // Unwrap the pin to get a reference to the value
    let r: &mut u8 = Pin::into_inner(pinned);
    // assert_eq!(*r, 5);
    println!("{}: {:p}", *r, &r);
    // println!("pinned: {:p}", &pinned);
    println!("val: {:p}", &val);
    // println!("pinned: {:p}", &pinned);

}
*/




/*
// https://doc.rust-lang.org/nightly/src/core/pin.rs.html#1667-1672
/// The simplest and most flexible way to pin a value that does not implement [`Unpin`] is to put
/// that value inside a [`Box`] and then turn that [`Box`] into a "pinning [`Box`]" by wrapping it
/// in a [`Pin`]. You can do both of these in a single step using [`Box::pin`]. Let's see an
/// example of using this flow to pin a [`Future`] returned from calling an `async fn`, a common
/// use case as described above.
///
use std::pin::Pin;
use std::future::Future;

async fn add_one(x: u32) -> u32 {
    x + 1
}
/// If you have a value which is already boxed, for example a [`Box<dyn Future>`][Box], you can pin
/// that value in-place at its current memory address using [`Box::into_pin`].
fn boxed_add_one(x: u32) -> Box<dyn Future<Output = u32>> {
    Box::new(add_one(x))
}

fn main() {
    // Call the async function to get a future back
    let fut = add_one(42);
   
    // Pin the future inside a pinning box
    let pinned_fut: Pin<Box<_>> = Box::pin(fut);

    let res = futures::executor::block_on(pinned_fut);
    println!("{}", res);


    let boxed_fut: Box<dyn Future<Output = u32>> = boxed_add_one(42);
 
    // Pin the future inside the existing box
    let pinned_fut: Pin<Box<_>> = Box::into_pin(boxed_fut);

    let res = futures::executor::block_on(pinned_fut);
    println!("{}", res);
    
}
*/




/*
// https://blog.yoshuawuyts.com/async-cancellation-1/#tasks-and-futures
// 1. Cancel a future before it starts executing
// Here we create a drop guard, pass it to an async function which returns a future, and then drop the future before we .await it:
use std::time::Duration;

struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        println!("2");
    }
}

async fn foo(guard: Guard) {
    println!("3");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("4");
}

fn main() {
    println!("1");
    let guard = Guard {};
    let fut = foo(guard);
    drop(fut);
    println!("done");
}
*/






/*
use futures_util::FutureExt;

pub async fn foo() {
    let fut = async {};
    fut.await;
    // drop(fut); // error[E0382]: use of moved value: `fut`
}
// source
async fn example(x: &str) -> usize {
    x.len()
}
// roughly equivalent to Desugared (only awaitable in tokio runtime):
// async unsafe fn example<'a>(x: &'a str) -> impl std::future::Future<Output = usize> + 'a {
//     async move { x.len() }
// }
async fn bar() -> Result<(), Box<dyn std::error::Error + Send>> {
    println!("Hello world");
    Ok(())
}
fn some_async_fn() -> futures::future::BoxFuture<'static, String> {
    std::future::ready("test".to_owned()).boxed()
}
#[tokio::main]
async fn main() {
    let main_time = tokio::time::Instant::now();


    let fut = foo();
    let _ = futures::executor::block_on(fut);

    let fut = bar();
    let _ = futures::executor::block_on(fut);

    let fut = some_async_fn();
    let res: String = futures::executor::block_on(fut);
    println!("{}", res);


    unsafe {
        let fut = example("hello");
        
        let res = futures::executor::block_on(fut);
        println!("{}", res);
    }
    

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/




/*
// override in Rust (documented)
fn the_default() {
    println!("default implementation");
}

trait Foo {
    fn method(&self) {
        the_default()    
    }
}

struct AStruct;
impl Foo for AStruct {}

struct BStruct;
impl Foo for BStruct {
    fn method(&self) {
        the_default();
        println!("BStruct's own implentation");
    }
}
struct CStruct;
impl Foo for CStruct {
    fn method(&self) {
        // the_default();
        println!("CStruct's own implentation");
    }
}

fn main() {
    let main_time: tokio::time::Instant = tokio::time::Instant::now();

    let a = AStruct;
    a.method();

    let b = BStruct;
    b.method();

    let c = CStruct;
    c.method();

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
// output:
// default implementation
// BStruct's own implentation
// CStruct's own implentation

// ⌛️ Execution time: 26.558µs (0.026558 ms)
*/




/*
// 定义一个 trait Foo，它包含一个方法 do_something
trait Foo {
    fn do_something(&self);
}

// 定义一个结构体 MyStruct，用于实现 Foo trait 的方法
struct MyStruct;

// 为 MyStruct 实现 Foo trait 的 do_something 方法
impl Foo for MyStruct {
    fn do_something(&self) {
        println!("Doing something");
    }
}

fn main() {
    let main_time = tokio::time::Instant::now();


    // 创建一个 Box 智能指针，指向实现了 Foo trait 的类型 MyStruct 的堆分配实例
    let my_box: Box<dyn Foo> = Box::new(MyStruct);
    
    // 调用 Box 中存储的对象的 do_something 方法
    my_box.do_something();

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

// 作者：Pomelo_刘金
// 链接：https://juejin.cn/post/7264921108410564666
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/





/*
// use std::net::{TcpListener, TcpStream};
use tokio::net::{TcpListener, TcpStream};
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr : std::net::SocketAddr = "127.0.0.1:33333".parse().unwrap();

    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    });
    let _listenerTask = tokio::spawn(async move{
        // Create a TcpListener (use tokio one so that it's awaitable)
        let listener = TcpListener::bind(server_addr).await.unwrap();
        println!("Server listening on http://{}", server_addr);

        while let Ok((mut stream, addr)) = listener.accept().await {
            tokio::spawn(async move {
                println!("tcp accept from {:?}", addr);
                let mut buf = [0; 1024];
                let size: usize = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await.unwrap();
                println!("received from remote {} bytes data.", size);

                // Immediately respond to the client
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: 55\r\n\r\nImmediate response, my tasks still run in background :>");

                // Write the response to the socket
                if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await {
                    eprintln!("Error writing to socket: {}", e);
                }
            });
        }
    }); // put `.await` here will forever prevent the next task (which is Tcp Client) from spawning !
    //  but without `.await`, after it received one client request, program terminate.

    /*
    let server_addr : std::net::SocketAddr = "127.0.0.1:33333".parse().unwrap();
    // let server_addr = "127.0.0.1:33333";
    tokio::spawn(async move {
        match TcpListener::bind(&server_addr).await {
            Ok(listener) => {
                println!("Listening on {}", server_addr);
                // let (mut stream, addr) = listener.accept().await.unwrap();   // will terminate shortly
                while let Ok((mut stream, addr)) = listener.accept().await {
                    tokio::spawn(async move {
                        // Immediately respond to the client
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: 55\r\n\r\nImmediate response, my tasks still run in background :>");

                        // Write the response to the socket
                        if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await {
                            eprintln!("Error writing to socket: {}", e);
                        }

                        println!("tcp accept from {:?}", addr);
                        let mut buf = [0; 1024];
                        let size: usize = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await.unwrap();
                        println!("received from remote {} bytes data.", size);
                    });
                }
                
                // std::thread::sleep(std::time::Duration::from_millis(1000));
                // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            Err(err) => {
                println!("Error binding address: {}", err);
            }
        };
    });
    */
	
	// std::thread::sleep(std::time::Duration::from_millis(3*1000));
    // tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;


    let addrs = [
        std::net::SocketAddr::from(([127, 0, 0, 1], 8080)),
        std::net::SocketAddr::from(([127, 0, 0, 1], 8081)),
        // std::net::SocketAddr::from(([127, 0, 0, 1], 33333)),
    ];

    // tokio::spawn(async move {
    //     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    //     // let mut stream: TcpStream = TcpStream::connect("127.0.0.1:33333").unwrap();
    //     if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
    //         println!("Connected to the server!");
    //         let n: usize = stream.write(&[1,2,3,4,5,6,7,8,9,10]).unwrap();
    //         println!("send {} bytes to remote node, waiting for end.", n);
    //     } else {
    //         println!("Couldn't connect to server...");
    //     }
        
    //     // std::thread::sleep(std::time::Duration::from_millis(1000));
    // });

    let connect_task = tokio::spawn(async {
        println!("Client task spawned");
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        match TcpStream::connect("127.0.0.1:33333").await {
            Ok(mut stream) => {
                println!("Connected to the server!");
                let n: usize = tokio::io::AsyncWriteExt::write(&mut stream, &[1,2,3,4,5,6,7,8,9,10]).await.unwrap();
                // tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                
                // stream.read(&mut [0; 128]);
                println!("send {} bytes to remote node, waiting for end.", n);
                // loop {
                    match tokio::io::AsyncWriteExt::write(&mut stream, &[1,2]).await {
                        Ok(bytes) => {
                            println!("Stream bytes written: {}", bytes);
                        }
                        Err(err) => println!("Error writing stream: {}", err)
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                // }
            }
            Err(err) => {
                eprintln!("Couldn't connect to server: {:?}", err);
            }
        }
    }).await;   //  without `.await`, it will skip any async/await operations inside this task,
    //              only "Client task spawned" got printed

    // std::thread::sleep(std::time::Duration::from_millis(10*60*1000));
    // tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    Ok(())

}

// 作者：chirpyli
// 链接：https://juejin.cn/post/6844903880501755917
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/





/*
// still learning TcpStream:    https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html
// async tokio:                 https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html
use std::io::{self, Read};
// use std::net::TcpStream;

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr : std::net::SocketAddr = "127.0.0.1:7878".parse().unwrap();

    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    });
    let _listenerTask = tokio::spawn(async move{
        // Create a TcpListener (use tokio one so that it's awaitable)
        let listener = TcpListener::bind(server_addr).await.unwrap();
        println!("[TcpListener] Server listening on http://{}", server_addr);

        while let Ok((mut stream, socket_addr)) = listener.accept().await {

            tokio::spawn(async move {
                println!("[TcpListener] Accepted a connection from {:?}", socket_addr);

                let mut buf = [0; 1024];
                let size: usize = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await.unwrap();

                println!("\n[TcpListener] Request data: {}\n", String::from_utf8_lossy(&buf[..]));
                println!("[TcpListener] Received from remote {} bytes data.", size);

                // Immediately respond to the client
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: 55\r\n\r\nImmediate response, my tasks still run in background :>");

                // Write the response to the socket
                match tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await {
                    Ok(_) => {
                        // stream.flush().unwrap(); // std::net
                        let _flush_res = tokio::io::AsyncWriteExt::flush(&mut stream).await;
                        println!("[TcpListener] Successfully write_all");
                    },
                    Err(e) => {
                        eprintln!("[TcpListener] Error writing to socket: {}", e);
                    }
                }
            });
        }
    }); // put `.await` here will forever prevent the next task (which is Tcp Client) from spawning !
    //  but without `.await`, after it received one client request, program terminate.


    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
    // streamTask
    let mut stream = TcpStream::connect("127.0.0.1:7878").await
        .expect("Couldn't connect to the server...");

    println!("[TcpStream] Successfully connected");

    // stream.set_nonblocking(true).expect("set_nonblocking call failed");

    // let mut buf = vec![];
    // loop {
    //     // std::net's way:
    //     // match stream.read_to_end(&mut buf) {
    //     //     Ok(_) => break,
    //     //     Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    //     //         // wait until network socket is ready, typically implemented
    //     //         // via platform-specific APIs such as epoll or IOCP
    //     //         // wait_for_fd();
    //     //     }
    //     //     Err(e) => panic!("encountered IO error: {e}"),
    //     // };

    //     // tokio::net's way: using fn poll_peek(&self,cx: &mut Context<'_>,buf: &mut ReadBuf<'_>) -> Poll<Result<usize>>
    //     // let mut buf = [0; 10];
    //     let mut buf = tokio::io::ReadBuf::new(&mut buf);

    //     std::future::poll_fn(|cx| {
    //         stream.poll_peek(cx, &mut buf)  // <--
    //     }).await.unwrap();

    //     println!("bytes: {buf:?}");
    // };
    // println!("bytes: {buf:?}");


    loop {
        
        // Wait for the socket to be readable
        stream.readable().await?;

        // Creating the buffer **after** the `await` prevents it from
        // being stored in the async task.
        let mut buf = [0; 4096];
        // let mut buf = Vec::with_capacity(4096);  // vector for `try_read_buf()`


        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }

        // Immediately respond to the client
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: 55\r\n\r\nHello server :>");

        // Write the response to the socket
        match tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await {
            Ok(_) => {
                // stream.flush().unwrap(); // std::net
                let _flush_res = tokio::io::AsyncWriteExt::flush(&mut stream).await;
                println!("[TcpStream] Successfully write_all");
            },
            Err(e) => {
                eprintln!("[TcpStream] Error writing to socket: {}", e);
            }
        }
        

        /*
        // https://stackoverflow.com/questions/71365810/how-to-share-tokionettcpstream-act-on-it-concurrently
        // let mut stream = common_stream.lock().unwrap();
        let mut buf = [0u8; 10];
        // stream.try_read(&mut buf).unwrap();
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
        buf.reverse();
        stream.write_all(&buf).await.unwrap();
        let (read, write) = stream.split();

        match stream.flush().await {
            Ok(_) => {},
            Err(err) => {
                println!("Error flushing stream: {:?}", err);
            }
        };
        */

    }

    Ok(())
}
*/





// Problem: future created by async block is not `Send`
// https://stackoverflow.com/questions/75079221/future-created-by-async-block-is-not-send?rq=1
use std::{cell::RefCell, rc::Rc};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct SharedString {
    state: Rc<RefCell<String>>,
}

impl SharedString {
    pub fn new(initial: &str) -> Self {
        Self {
            state: Rc::new(RefCell::new(initial.into())),
        }
    }
}

async fn run() {
    let shared_string = SharedString::new("Hello,");
    sleep(Duration::from_millis(1)).await;
    *shared_string.state.borrow_mut() += " world!";
    sleep(Duration::from_millis(1)).await;
    println!("{:?}", shared_string.state.borrow());
}

#[tokio::main]
async fn main() {
    tokio::task::spawn(run()).await.unwrap();
}






/*
use std::sync::Arc;
use tokio::{runtime::Builder, sync::Mutex};

#[tokio::main]
async fn main() {
    let result = Arc::new(Mutex::new(Vec::<usize>::new()));
    // let runtime = Builder::new_multi_thread()
    //     .worker_threads(1)
    //     .enable_all()
    //     .build()
    //     .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let result_arc = result.clone();
        // handles.push(runtime.spawn(async move {
        //     let ret = my_bg_task(i).await;
        //     let mut copy = result_arc.lock().await;
        //     copy.push(ret);
        // }));

        handles.push(tokio::spawn(async move {
            let ret = my_bg_task(i).await;
            let mut copy = result_arc.lock().await;
            copy.push(ret);
        }));
    }

    // Wait for all of them to complete.
    for handle in handles {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
        // runtime.block_on(handle).unwrap();
        handle.await.unwrap();
    }

    let inner: Vec<_> = Arc::try_unwrap(result).unwrap().into_inner();
    println!("{inner:?}");
}

async fn my_bg_task(i: usize) -> usize {
    i * 2
}
*/





/*
// use std::net::TcpListener;
use tokio::net::TcpListener;

async fn handle_connection_async(mut stream: tokio::net::TcpStream, _socket_addr : std::net::SocketAddr) {
    // 从连接中顺序读取 1024 字节数据 (Sequentially reads 1024 bytes of data from the connection)
    let mut buffer: [u8; 1024] = [0; 1024];
    tokio::io::AsyncReadExt::read(&mut stream, &mut buffer).await.unwrap();

    let status_line: String = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    let contents: String = "hello".to_string();

    // 将回复内容写入连接缓存中 (Write the reply content to the connection cache)
    let response = format!("{status_line}{contents}");
    tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await.unwrap();
    // 使用 flush 将缓存中的内容发送到客户端 (Use flush to send the contents of the cache to the client)
    tokio::io::AsyncWriteExt::flush(&mut stream).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    
    // (std::net::TcpListener::incomming()) Iterating over it 
    // is equivalent to calling TcpListener::accept in a loop.
    while let Ok((stream, socket_addr)) = listener.accept().await {
        // do something with the TcpStream
        handle_connection_async(stream, socket_addr).await;
    }
}
*/





/*
// https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html
use tokio::runtime::Runtime;

fn function_that_spawns(msg: String) {
    // Had we not used `rt.enter` below, this would panic.
    tokio::spawn(async move {
        println!("{}", msg);
    });
}

fn main() {
    let rt = Runtime::new().unwrap();

    let s = "Hello World!".to_string();

    // By entering the context, we tie `tokio::spawn` to this executor.
    let _guard = rt.enter();
    function_that_spawns(s);
}
*/




// require crate async_stream
// use tokio::sync::mpsc;

// fn main() {
//     let (tx, mut rx) = mpsc::channel::<usize>(16);

//     let stream = async_stream::stream! {
//         while let Some(item) = rx.recv().await {
//             yield item;
//         }
//     };
// }





/*
// Tokio simple TCP server
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => {
                        println!("socket hang up.");
                        return
                    },
                    Ok(n) => {
                        println!("read {} bytes", n);
                        n
                    },
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
*/




/*
// Highe level (answered by Shepmaster)
// https://stackoverflow.com/questions/41932137/what-is-the-best-approach-to-encapsulate-blocking-i-o-in-future-rs
use futures::future; // 0.3.15
use std::{thread, time::Duration};
use tokio::task; // 1.7.1, features = ["full"]

async fn delay_for(seconds: u64) -> Result<u64, task::JoinError> {
    task::spawn_blocking(move || {
        thread::sleep(Duration::from_secs(seconds));
        seconds
    })
    .await?;
    Ok(seconds)
}

#[tokio::main]
async fn main() -> Result<(), task::JoinError> {
    let a = delay_for(2);
    let b = delay_for(1);

    let (a, b) = future::join(a, b).await;
    let c = a? + b?;

    println!("{}", c);

    Ok(())
}
*/





// failed (chatgpt) try implementing own future awaitable task
/*
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::future::{ready, Ready};

struct CustomTask;

impl CustomTask {
    // Some operation that returns a result asynchronously
    fn perform_async_operation(&self) -> Ready<&'static str> {
        ready("Async operation completed")
    }
}

impl futures::Future for CustomTask {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // In a real-world scenario, you might perform some asynchronous
        // operation here and return Poll::Pending until it completes.
        // For simplicity, we're using a synchronous ready future here.
        Poll::Ready(self.perform_async_operation().await)
    }
}

#[tokio::main]
async fn main() {
    // Create an instance of CustomTask
    let custom_task = CustomTask;

    // Await the custom task
    let result = custom_task.await;

    // Print the result
    println!("Result: {}", result);
}
*/




/*
// https://stackoverflow.com/questions/74586510/implement-future-trait-based-on-future-available-inside-the-struct?rq=3
use futures::task::{Context, Poll};
use futures::Future;
use std::pin::Pin;
use tokio::time::{sleep, Sleep, Duration};

struct DelayedValue<T> {
    value: T,
    sleep: Sleep,
}
pin_project::pin_project! {
    struct DelayedValue<T> {
        value: Option<T>,
        #[pin]
        sleep: Sleep,
    }
}

impl<T> Future for DelayedValue<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.sleep.poll(cx) {
            Poll::Ready(()) => Poll::Ready(this.value.take().unwrap()),
            Poll::Pending => Poll::Pending,
        }
    }
}
// impl<T> Future for DelayedValue<T> {
//     type Output = T;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match &mut self.sleep.poll(cx) {
//             Poll::Ready(()) => Poll::Ready(self.value),
//             x => x,
//         }
//     }
// }

#[tokio::main]
async fn main() {
    let dv = DelayedValue {
        value: 10_u8,
        sleep: sleep(Duration::from_millis(5000)),
    };

    println!("waiting for delayed value");
    
    let v = dv.await;
    println!("delayed value: {}", v);
}
*/





// problem: https://stackoverflow.com/questions/68448854/how-to-await-for-the-first-k-futures?rq=3
// You can use streams (async iterators) for this. 
// You can use FuturesUnordered as an unordered collection of futures, 
// which can be used as a stream where you get each future's result in the order they complete. 
// Then you can combine this with .take(n) to only take the first n items of the stream, and then .collect::<Vec<_>>() to wait for the stream to finish and collect the results in a Vec:
/*
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let futures = vec![
        // f(n, t): wait for `t` millis, then return `n`
        f(1, 1000),
        f(2, 10),
        f(3, 105),
        f(4, 40),
        f(5, 70),
        f(6, 270),
    ];

    // create unordered collection of futures
    let futures = futures.into_iter().collect::<FuturesUnordered<_>>();

    // use collection as a stream, take only first 4 and collect into a `Vec`
    let first_4 = futures.take(4).collect::<Vec<_>>().await;

    // note: any remaining futures will be cancelled automatically when the
    // stream is consumed

    // check with expected result, based on the order of completion
    assert_eq!(first_4, vec![2, 4, 5, 3]);
}

async fn f(n: u64, t: u64) -> u64 {
    sleep(Duration::from_millis(t)).await;
    n
}






// Edit: If you want to also get the index of completed the future, you can use this:
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let futures = vec![
        // f(n, t): wait for `t` millis, then return `n`
        f('a', 1000),
        f('b', 10),
        f('c', 105),
        f('d', 40),
        f('e', 70),
        f('f', 270),
    ];

    // create unordered collection of futures with indices
    let futures = futures
        .into_iter()
        .enumerate()
        .map(|(i, fut)| fut.map(move |res| (i, res)))
        .collect::<FuturesUnordered<_>>();

    // use collection as a stream, take only first 4 to complete and collect
    // into a `Vec`
    let first_4 = futures.take(4).collect::<Vec<_>>().await;

    // note: any remaining futures will be cancelled automatically when the
    // stream is consumed

    // check with expected result, based on the order of completion
    assert_eq!(first_4, vec![(1, 'b'), (3, 'd'), (4, 'e'), (2, 'c')],);
}

async fn f<N>(n: N, t: u64) -> N {
    sleep(Duration::from_millis(t)).await;
    n
}
*/




/*
// solution 2:

// I don't believe there is anything built for this purpose. 
// Perhaps you can do this with Streams or Channels, 
// but the join_all implementation isn't too complicated.
// I've modified it so that it only waits for n results:
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::MaybeDone; // 0.3.15

fn iter_pin_mut<T>(slice: Pin<&mut [T]>) -> impl Iterator<Item = Pin<&mut T>> {
    // Safety: `std` _could_ make this unsound if it were to decide Pin's
    // invariants aren't required to transmit through slices. Otherwise this has
    // the same safety as a normal field pin projection.
    //
    // Copied from `futures` implementation of `join_all`.
    unsafe { slice.get_unchecked_mut() }.iter_mut().map(|t| unsafe { Pin::new_unchecked(t) })
}

#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct JoinSome<F>
where
    F: Future,
{
    elems: Pin<Box<[MaybeDone<F>]>>,
    count: usize,
}

/// Will wait for at least `n` futures to complete. More may be returned if
/// multiple resolve around the same time.
///
/// # Panics
///
/// Will panic if iterator doesn't contain at least `n` futures.
pub fn join_some<I>(i: I, n: usize) -> JoinSome<I::Item>
where
    I: IntoIterator,
    I::Item: Future,
{
    let elems: Box<[_]> = i.into_iter().map(MaybeDone::Future).collect();
    assert!(elems.len() >= n);
    JoinSome { elems: elems.into(), count: n }
}

impl<F> Future for JoinSome<F>
where
    F: Future,
{
    type Output = Vec<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut num_done = 0;

        for elem in iter_pin_mut(self.elems.as_mut()) {
            if !elem.poll(cx).is_pending() {
                num_done += 1;
            }
        }

        if num_done >= self.count {
            let mut elems = std::mem::replace(&mut self.elems, Box::pin([]));
            let result = iter_pin_mut(elems.as_mut()).filter_map(|e| e.take_output()).collect();
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}

// I've added documentation in the source that explains its behavior. 
// You can see that it works with this simple test program:
#[tokio::main]
async fn main() {
    use std::time::{Instant, Duration};
    use tokio::time::sleep;

    let futures = vec![
        sleep(Duration::from_secs(1)),
        sleep(Duration::from_secs(2)),
        sleep(Duration::from_secs(3)),
        sleep(Duration::from_secs(4)),
        sleep(Duration::from_secs(5)),
    ];
    
    let now = Instant::now();
    let some = join_some(futures, 3).await;
    let elapsed = now.elapsed();
    
    println!("{} results in {:.2?}", some.len(), elapsed);
}
*/




/*
// documented
// https://stackoverflow.com/questions/63434977/how-can-i-spawn-asynchronous-methods-in-a-loop?rq=3
// The reference is passed to tokio::spawn() which hands it off to another thread, and the compiler cannot prove that item will outlive that thread. (The same kind of problem is encountered when you want to send reference to local data to a thread.)
// There are several possible solutions to this; the one I find most elegant is to move items into the async closure passed to tokio::spawn(), and have the task hand them back to you once it's done. Basically you consume the items vector to create the tasks and immediately reconstitute it from the awaited results:
use std::time::Duration;
use tokio;

struct Item {
    resolved: bool,
}

impl Item {
    async fn resolve(&mut self) {
        tokio::time::sleep(Duration::from_millis(100)).await;
        self.resolved = true;
    }

    fn print_result(&self) {
        println!("resolved={}", self.resolved);
    }
}

async fn join_parallel<T: Send + 'static>(
    futs: impl IntoIterator<Item = impl std::future::Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let tasks: Vec<_> = futs.into_iter().map(tokio::spawn).collect();
    // unwrap the Result because it is introduced by tokio::spawn()
    // and isn't something our caller can handle
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

#[tokio::main]
async fn main() {
    let items: Vec<_> = (0..4).map(|_| Item { resolved: false }).collect();

    // note the use of `into_iter()` to consume `items`
    // let tasks: Vec<_> = items
    //     .into_iter()
    //     .map(|mut item| {
    //         tokio::spawn(async {
    //             item.resolve().await;
    //             item
    //         })
    //     })
    //     .collect();
    // // await the tasks for resolve's to complete and give back our items
    // let mut items = vec![];
    // for task in tasks {
    //     items.push(task.await.unwrap());
    // }
    // // verify that we've got the results
    // for item in items.iter() {
    //     item.print_result();
    // }

    let items: Vec<Item> = join_parallel(items.into_iter().map(|mut item| async {
        item.resolve().await;
        item
    })).await;
    for item in &items {
        item.print_result();
    }

}
*/

 



// good question & answer (documented)
// https://stackoverflow.com/questions/70591386/calling-a-generic-async-function-with-a-mutably-borrowed-argument?rq=3

// working code
// use std::future::Future;
// use std::pin::Pin;

// trait AsyncSingleArgFnOnce<Arg>: FnOnce(Arg) -> <Self as AsyncSingleArgFnOnce<Arg>>::Fut {
//     type Fut: Future<Output = <Self as AsyncSingleArgFnOnce<Arg>>::Output>;
//     type Output;
// }

// impl<Arg, F, Fut> AsyncSingleArgFnOnce<Arg> for F
// where
//     F: FnOnce(Arg) -> Fut,
//     Fut: Future,
// {
//     type Fut = Fut;
//     type Output = Fut::Output;
// }
// async fn call_changer<F>(changer: F)
// where
//     F: for<'a> AsyncSingleArgFnOnce<&'a mut i32, Output = ()>,
// {
//     let mut i = 0;
//     changer(&mut i).await;
//     dbg!(i);
// }

// async fn call_changer<F>(changer: F)
// // where
// //     F: for<'a> FnOnce(&'a mut i32) -> Pin<Box<dyn Future<Output = ()> + 'a>>,

// // where
// //     F: FnOnce(&mut i32) -> Pin<Box<dyn Future<Output = ()> + '_>>,

// // does not work !
// // where
// //     for<'a> FnOnce(&'a mut i32) -> (Fut + 'a),
// //     Fut: Future<Output = ()>,

// {
//     let mut i = 0;
//     changer(&mut i).await;
//     dbg!(i);
// }

// #[tokio::main]
// async fn main() {
//     call_changer(|i| {
//         Box::pin(async move {
//             *i = 100;
//         })
//     })
//     .await;
// }

// #[tokio::main]
// async fn main() {
//     let main_time: tokio::time::Instant = tokio::time::Instant::now();

//     async fn callback(i: &mut i32) {
//         *i += 100;
//     }
//     call_changer(callback).await;

//     // End of main
//     let duration: std::time::Duration = main_time.elapsed();
//     let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
//     println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
// }




/*
// not-documented
// https://users.rust-lang.org/t/how-to-return-an-async-closure-from-a-function/81999/4
use std::{future::Future, pin::Pin, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, ptr};

async fn async_add(x: i32, y: i32) -> i32 {
    x + y
}

fn make_async_adder(x: i32) -> impl Fn(i32) -> Pin<Box<dyn Future<Output = i32>>> {
    move |y: i32| Box::pin(async_add(x, y))
}

// tuanhayho's old code appliance
// by this approach, no need for `tokio` async main, just raw std
// An async function is really just a regular function that returns something that implements std::future::Future
fn drive_to_completion<F>(f: F) -> F::Output
where
    // F: Future,                  // works
    // F: Future<Output = ()>,     // not works
    // F: Future<Output = i32>,    // also works
    // F: dyn Future
{
    let waker = my_waker();
    let mut context = Context::from_waker(&waker);

    let mut t = Box::pin(f);
    let t = t.as_mut();

    loop {
        match t.poll(&mut context) {
            Poll::Ready(v) => return v,
            Poll::Pending => panic!("This executor does not support futures that are not ready"),
        }
    }
}

type WakerData = *const ();

unsafe fn clone(_: WakerData) -> RawWaker {
    my_raw_waker()
}
unsafe fn wake(_: WakerData) {}
unsafe fn wake_by_ref(_: WakerData) {}
unsafe fn drop(_: WakerData) {}

static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

fn my_raw_waker() -> RawWaker {
    RawWaker::new(ptr::null(), &MY_VTABLE)
}

fn my_waker() -> Waker {
    unsafe { 
	    Waker::from_raw(my_raw_waker()) 
    }
}

// #[tokio::main]=
fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let async_adder = make_async_adder(2);

    // dbg!(async_adder(40).await);
    // dbg!(drive_to_completion(async_adder(40)));
    drive_to_completion(async_adder(40));

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/




/*
// How to tokio::join multiple tasks?
// join_all and try_join_all, as well as more versatile FuturesOrdered and FuturesUnordered utilities from the same crate futures, are executed as a single task. This is probably fine if the constituent futures are not often concurrently ready to perform work, but if you want to make use of CPU parallelism with the multi-threaded runtime, consider spawning the individual futures as separate tasks and waiting on the tasks to finish.
// https://stackoverflow.com/questions/63589668/how-to-tokiojoin-multiple-tasks?noredirect=1&lq=1
// edited Dec 16, 2022 at 6:48 | answered Oct 3, 2021 at 11:51 by mzabaluev
use futures::stream::FuturesUnordered;
// use tokio::task::JoinSet;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // let mut set = JoinSet::new();
    
    let tasks = (0..5).map(|i| tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await; // simulate some work
        i * 2
    })).collect::<FuturesUnordered<_>>();

    // `futures::future::join_all()` approach:
    // let result = futures::future::join_all(tasks).await;

    // Old API: Spawn tasks with tokio::spawn and wait on the join handles:
    // let result = futures::future::try_join_all(tasks.into_iter().map(tokio::spawn)).await;

    // You can also use the FuturesOrdered and FuturesUnordered combinators to process the outputs asynchronously in a stream:
    let mut completion_stream = tasks.into_iter()
        .map(tokio::spawn)
        .collect::<FuturesUnordered<_>>();
    while let Some(result) = completion_stream.next().await {
        // ...    
        println!("{:?}", result); // [Ok(8), Ok(6), Ok(4), Ok(2), Ok(0)]
    }
    // let result = set.join_next().await {

    // }
    // println!("{:?}", result); // [Ok(8), Ok(6), Ok(4), Ok(2), Ok(0)]

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
// One caveat with waiting for tasks this way is that the tasks are not cancelled when the future (e.g. an async block) that has spawned the task and possibly owns the returned JoinHandle gets dropped. The JoinHandle::abort method needs to be used to explicitly cancel the task.
*/





/*
// not done researching yet
// https://stackoverflow.com/questions/60561573/how-can-one-await-a-result-of-a-boxed-future?rq=3
use futures::{future, Future};

use std::marker::Unpin;

fn test() -> Box<dyn Future<Output = Result<bool, ()>> + Unpin> {
    Box::new(future::ok(true))
}

async fn async_fn() -> bool {
    let result: bool = test().await.unwrap();
    return result;
}
fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let fut = async_fn();
    let res = futures::executor::block_on(fut);

    println!("res: {}", res);

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/





/*
// from single processing to parallel (basic, but still worth taking note)
// https://stackoverflow.com/questions/73055125/how-do-i-run-a-future-without-awaiting-it-in-rust?rq=3
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Player {
    name: String,
}
struct Client {}

impl Client {
    async fn init(&self) {}

    async fn get_player(&self, name: String, _now: Instant) -> Option<Player> {
        // Dummy code her that simulates a delay of 1 second
        tokio::time::sleep(Duration::from_millis(1000)).await;
        Some(Player { name })
    }
}

static client: Client = Client {};

#[tokio::main]
async fn main() {
    let begin = Instant::now();
    client.init().await;

    let get_player_futures = (0..10).into_iter().map(|i| async move {
        let now = Instant::now();
        let player = client
            .get_player(format!("Player #{}", i), now)
            .await
            .expect("panic");
        println!(
            "[{} ms] Retreived player: {:?}",
            begin.elapsed().as_millis(),
            player.name
        );
    });

    futures::future::join_all(get_player_futures).await;
}
*/



// fn main() {
//     println!("Hello, Rust on Macbook\n");
//     std::thread::sleep(std::time::Duration::from_millis(100));
//     for i in 0..=10 {
//         println!("Count: {}", i);
//         std::thread::sleep(std::time::Duration::from_millis(100));
//     }
// }





/*
// ARM register MOV instruction stimulator
fn main() {
    // Declare and initialize variables
    let r0: i64 = 42;
    let mut r1: i64 = 0;

    // Print the initial values
    println!("Initial values:");
    println!("R0: {}", r0);
    println!("R1: {}", r1);

    // Move the value from R0 to R1
    r1 = r0;

    // Print the values after the move
    println!("\nValues after the move:");
    // Uncommenting the next line would result in a compile error
    // println!("R0: {}", r0);
    println!("R1: {}", r1);
}
*/




/* 
// chatgpt failure still in tcp stuff
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;
// use tokio::sync::Mutex;
use std::sync::Mutex;

#[tokio::main]
async fn main() {
    // Create a TcpListener that will listen for incoming connections
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    // Create a HashMap to store the connected clients
    let clients: HashMap<String, mpsc::Sender<String>> = HashMap::new();

    // Create a channel to communicate with the listener task
    let (sender, mut receiver) = mpsc::channel::<String>(100);

    // Spawn a task to handle incoming connections
    tokio::spawn(handle_connections(listener, clients.clone(), sender));

    // Spawn a task to handle messages from clients
    tokio::spawn(handle_messages(clients, receiver));
}

async fn handle_connections(
    listener: TcpListener,
    mut clients: HashMap<String, mpsc::Sender<String>>,
    sender: mpsc::Sender<String>,
) {
    while let Ok((stream, _)) = listener.accept().await {
        // Spawn a task to handle each incoming connection
        tokio::spawn(handle_client(stream, clients.clone(), sender.clone()));
    }
}

async fn handle_client(
    mut stream: TcpStream,
    // mut clients: HashMap<String, mpsc::Sender<String>>,
    clients: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>,
    sender: mpsc::Sender<String>,
) {
    // Read the username from the client
    let mut username = String::new();
    stream.read_to_string(&mut username).await.unwrap();

    // Create a channel for this client and store it in the HashMap
    let (client_sender, mut client_receiver) = mpsc::channel::<String>(100);
    clients.lock().unwrap().insert(username.clone(), client_sender);

    // Notify other clients about the new user
    sender.send(format!("{} has joined the chat", username)).await.unwrap();

    // // Spawn a task to handle outgoing messages to this client
    // tokio::spawn(handle_outgoing_messages(stream, client_receiver));

    // // Read incoming messages from this client and broadcast them to others
    // // while let Some(message) = client_receiver.recv().await {
    // while let Some(message) = client_receiver.clone().recv().await {
    //     let message = format!("{}: {}", username, message);
    //     sender.send(message.clone()).await.unwrap();

    //     // Optionally, you can handle special commands here (e.g., "/quit" to disconnect)
    // }

    // // Remove the client from the HashMap when they disconnect
    // clients.remove(&username);

    // Create a cloneable reference to the client's receiver using Arc and Mutex
    let client_receiver = Arc::new(Mutex::new(client_receiver));
    let mut recv_inner = client_receiver.lock().unwrap();


    // Spawn a task to handle outgoing messages to this client
    tokio::spawn(async move {
        let recv_clone = Arc::clone(&client_receiver).into_inner();
        handle_outgoing_messages(stream, recv_clone.unwrap());
    });

    // Read incoming messages from this client and broadcast them to others
    while let Some(message) = client_receiver.lock().unwrap().recv().await {
        let message = format!("{}: {}", username, message);
        sender.send(message.clone()).await.unwrap();

        // Optionally, you can handle special commands here (e.g., "/quit" to disconnect)
    }

    // Remove the client from the HashMap when they disconnect
    clients.lock().unwrap().remove(&username);
}

async fn handle_outgoing_messages(mut stream: TcpStream, mut receiver: mpsc::Receiver<String>) {
    while let Some(message) = receiver.recv().await {
        // Write the message to the client
        stream.write_all(message.as_bytes()).await.unwrap();
    }
}

async fn handle_messages(mut clients: HashMap<String, mpsc::Sender<String>>, mut receiver: mpsc::Receiver<String>) {
    while let Some(message) = receiver.recv().await {
        // Broadcast the message to all connected clients
        for client_sender in clients.values() {
            client_sender.send(message.clone()).await.unwrap();
        }
    }
}
*/
    







// https://stackoverflow.com/questions/76772782/how-do-i-fix-cannot-be-sent-between-threads-safely-when-using-tokiospawn-and
// https://stackoverflow.com/questions/65269738/spawn-non-static-future-with-tokio?rq=3
// https://stackoverflow.com/questions/60561573/how-can-one-await-a-result-of-a-boxed-future?rq=3
// https://stackoverflow.com/questions/61295176/how-to-implement-a-future-or-stream-that-polls-an-async-fn?rq=3
// https://stackoverflow.com/questions/67344330/awaiting-a-number-of-futures-unknown-at-compile-time?rq=3
// https://stackoverflow.com/questions/68448854/how-to-await-for-the-first-k-futures?rq=3
// https://stackoverflow.com/questions/77113869/how-can-i-poll-a-pinoptionboxdyn-future?noredirect=1&lq=1
// https://stackoverflow.com/questions/65921581/how-can-i-define-an-async-method-in-a-trait?noredirect=1&lq=1
// https://stackoverflow.com/questions/69543904/is-it-possible-to-use-async-trait-with-dynamic-dispatch?rq=3
// https://stackoverflow.com/questions/75403078/using-dyn-async-traits-with-async-trait-crate-in-spawned-tokio-task?rq=3
// https://stackoverflow.com/questions/75489442/how-to-store-a-list-of-closures-returning-a-future-and-share-it-between-threads?noredirect=1&lq=1
// https://stackoverflow.com/questions/57536217/manually-polling-streams-in-future-implementation?rq=3
// https://stackoverflow.com/questions/63804009/how-do-i-convert-a-stream-into-a-future?rq=3
// https://stackoverflow.com/questions/59982803/how-to-convert-a-future-into-a-stream
// https://stackoverflow.com/questions/42928432/how-to-select-between-a-future-and-stream-in-rust?noredirect=1&lq=1






// https://codereview.stackexchange.com/questions/141946/insertion-sort-in-rust/142070#142070
// pub fn insertion_sort<T>(values: &mut [T])
//     where T: Ord
// {
//     for i in 0..values.len() {
//         for j in (0..i).rev() {
//             if values[j] >= values[j + 1] {
//                 values.swap(j, j + 1);
//             } else {
//                 break
//             }
//         }
//     }
// }

// // #[macro_use]
// // extern crate quickcheck;

// #[test]
// fn test_insertion_sort_empty() {
//     let mut values: [i32; 0] = [];
//     insertion_sort(&mut values);
//     assert_eq!(values, [])
// }

// #[test]
// fn test_insertion_sort_one() {
//     let mut values = [1];
//     insertion_sort(&mut values);
//     assert_eq!(values, [1]);
// }

// #[test]
// fn test_insertion_multi() {
//     let mut values = [9, 8, 7, 11, 10];
//     insertion_sort(&mut values);
//     let values_expected: Vec<_> = (7..12).collect();
//     assert_eq!(values_expected, values);
// }

// // quickcheck! {
// //     fn test_insertion_everything(xs: Vec<i32>) -> bool {
// //         // Macro doesn't allow `mut` in the `fn` declaration :-(
// //         let mut xs = xs;

// //         let mut expected_sorted = xs.clone();
// //         expected_sorted.sort();

// //         insertion_sort(&mut xs);

// //         expected_sorted == xs
// //     }
// // }





// https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust/173437#173437
// use std::collections::HashMap;

// fn average(numbers: &[i32]) -> f32 {
//     numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
// }

// fn median(numbers: &mut [i32]) -> i32 {
//     numbers.sort();
//     let mid = numbers.len() / 2;
//     numbers[mid]
// }

// fn mode(numbers: &[i32]) -> i32 {
//     let mut occurrences = HashMap::new();

//     for &value in numbers {
//         *occurrences.entry(value).or_insert(0) += 1;
//     }

//     occurrences
//         .into_iter()
//         .max_by_key(|&(_, count)| count)
//         .map(|(val, _)| val)
//         .expect("Cannot compute the mode of zero numbers")
// }

// fn main() {
//     let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

//     println!("AVERAGE: {}", average(&numbers));
//     println!("MEDIAN: {}", median(&mut numbers));
//     println!("MODE: {}", mode(&numbers));
// }




// https://stackoverflow.com/questions/67373797/is-it-possible-to-await-a-dyn-future?rq=3
// trying to asynchronously invoke a Future that is returned from a function
// use core::future::Future;

// fn choose_your_adventure<'a>(i: usize) -> Box<&'a dyn Future<Output = ()>> {
//     match i {
//         0 => Box::new(&async {}),
//         _ => Box::new(&async {})
//     }
// }

// async fn does_not_work() -> () {
//     let choice = choose_your_adventure(0);
//     choice.await; // error[E0277]: `&dyn Future<Output = ()>` is not a future
// }
// #[tokio::main]
// async fn main() {
//     does_not_work.await;
// }





/*
// https://internals.rust-lang.org/t/how-to-talk-about-lifetime-of-opaque-fnonce-result/13783/2
pub trait BazInput<'a> {
    type Output: std::future::Future<Output = String>;
    
    fn get(self, s: &'a str) -> Self::Output;
}

impl<'a, F: FnOnce(&'a str) -> Fut, Fut: std::future::Future<Output = String>> BazInput<'a> for F {
    type Output = Fut;
    
    fn get(self, s: &'a str) -> Self::Output {
        self(s)
    }
}

async fn baz<F>(text: String, f: F) -> String
where
    for<'a> F: BazInput<'a>,
{
    f.get(&*text).await
}

async fn foo(text: &str) -> String {
    text.to_string()
}

async fn test() -> String {
    baz("some fucking text".to_string(), foo).await
}

#[tokio::main]
async fn main() {
    let res = test().await;
    println!("res: {}", res);
}
*/





/*
// asked & solved on 22-01-2024 (documented)
// https://users.rust-lang.org/t/how-to-time-futuresunordered-tasks/105665
// this can achieve parallel async tasks sing FuturesUnordered.
use std::time::Duration;
use tokio::time::sleep;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::pin::Pin;
use std::future::Future;

async fn task_one() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Task 1");
    sleep(Duration::from_millis(40)).await;
    println!("Task 1 Done");
    Ok(())
}
async fn task_two() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Task 2");
    sleep(Duration::from_millis(20)).await;
    println!("Task 2 Done");
    Ok(())
}
async fn task_three() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Task 3");
    sleep(Duration::from_millis(10)).await;
    println!("Task 3 Done");
    Ok(())
}

#[tokio::main]
async fn main() {
    
    println!("Begin.");
    let main_time: tokio::time::Instant = tokio::time::Instant::now();

    
    let mut jobs = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>>>>::new();
    jobs.push(Box::pin(task_one()));
    jobs.push(Box::pin(task_two()));
    jobs.push(Box::pin(task_three()));
     
    while let Some(result) = jobs.next().await {
        match result {
            Ok(_) => (),
            Err(_) => {
                println!("Error with task.");
            }
        }
    }


    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/




/*
// use std::thread::sleep as sync_sleep;
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // asynchronous call
    tokio::join!(async_read_one(), async_read_two());

    let end = Instant::now();
    println!("Running Time: {:#?}", end - start);
}

async fn async_read_one() {
    println!("Read One started");
    sleep(Duration::from_millis(20)).await;
    println!("Read One completed");
}

async fn async_read_two() {
    println!("Read two started");
    sleep(Duration::from_millis(40)).await;
    println!("Read two completed");
}
*/







/*
// tcp sample code from community:
use std::{thread, time::Duration};

use tokio::{
    io::{self, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    let (tx, _) = broadcast::channel(16);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        loop {
            // IMAGINE RANDOM BYTES
            tx2.send([64u8; 5]).ok();
            println!("{}", tx2.receiver_count());
            // thread::sleep(Duration::from_millis(1500));
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    });

    loop {
        let (client, _) = listener.accept().await?;
        let rx = tx.subscribe();
        process_client(client, rx).await;
    }
}

async fn process_client(mut client: TcpStream, mut receiver: Receiver<[u8; 5]>) {
    tokio::spawn(async move {
        println!("Connected");
        while let Ok(received) = receiver.recv().await {
            let res = client.write(&received).await;
            println!("{:?}", res);
        }
    });
}
*/




/*
// Alice blog post: https://ryhl.io/blog/async-what-is-blocking/
async fn rayon_parallel_sum(nums: Vec<i32>) -> i32 {
    let (send, recv) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Perform an expensive computation.
        let mut sum = 0;
        for num in nums {
            sum += num;
        }

        // Send the result back to Tokio.
        let _ = send.send(sum);
    });

    // Wait for the rayon task.
    recv.await.expect("Panic in rayon::spawn")
}

async fn tokio_parallel_sum(nums: Vec<i32>) -> i32 {
    let (send, recv) = tokio::sync::oneshot::channel();

    // Spawn a task on tokio.
    tokio::spawn(async move {
        // Perform an expensive computation.
        let mut sum = 0;
        for num in nums {
            sum += num;
        }

        // Send the result back to Tokio.
        let _ = send.send(sum);
    });

    // Wait for the tokio task.
    recv.await.expect("Panic in tokio::spawn")
}

#[tokio::main]
async fn main() {
    let nums = vec![1; 1024 * 1024];
    let nums_clone = nums.clone();

    println!("{}", rayon_parallel_sum(nums).await);

    println!("{}", tokio_parallel_sum(nums_clone).await);
    
}
*/




/*
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use std::sync::Arc;

pub fn run(f: Box<dyn Fn() -> Result<(), ()> + Send>) {
    f();
}


fn main() {
    let x = Arc::new(0);
    run(Box::new(move || {
        let rt = Runtime::new().unwrap();
        // each time the closure is called, it will pass a clone of the Arc
        // into the task, so it can be called more than once
        let x = x.clone();
        let _t = rt.block_on(async move {
            let y = x;
        });
        Ok(())
    }));
}
*/



/*
// https://codereview.stackexchange.com/questions/147670/hex-string-to-base64?rq=1
fn main() {
    let b64: String = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{}", b64);
}

/// Decode the input string from hex into individual bytes
fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    let input_chars: Vec<_> = hex_string.chars().collect();

    input_chars.chunks(2).map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap();
        let second_byte = chunk[1].to_digit(16).unwrap();
        ((first_byte << 4) | second_byte) as u8
    }).collect()
}

/// Encode the decoded bytes into Base64
fn bytes_to_base64(decoded_bytes: &[u8]) -> String {
    let mut output = String::new();
    let alphabet: Vec<_> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();

    for chunk in decoded_bytes.chunks(3) {
        let c0 = chunk[0];
        let b = (c0 & 0xFC) >> 2;
        output.push(alphabet[b as usize]);

        let mut b = (c0 & 0x03) << 4;

        if let Some(c1) = chunk.get(1) {
            b |= (c1 & 0xF0) >> 4;
            output.push(alphabet[b as usize]);

            let mut b = (c1 & 0x0F) << 2;

            if let Some(c2) = chunk.get(2) {
                b |= (c2 & 0xC0) >> 6;
                output.push(alphabet[b as usize]);

                let b = c2 & 0x3F;
                output.push(alphabet[b as usize]);
            } else {
                output.push(alphabet[b as usize]);
                output.push('=');
            }
        } else {
            output.push(alphabet[b as usize]);
            output.push_str("==");
        }
    }

    output
}


fn hex_to_base64(hex_string: &str) -> String {
    bytes_to_base64(&hex_to_bytes(hex_string))
}

#[test]
fn test_hex_to_base64() {
    assert_eq!(
        hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}
*/




/*
// https://codereview.stackexchange.com/questions/277488/implementing-ord-trait-for-an-iterator-wrapper?rq=1
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::iter::Peekable;
use std::ops::{Deref, DerefMut};

struct IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    iter: RefCell<Peekable<I>>,
}

impl<I, T> Eq for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
}

impl<I, T> PartialEq<Self> for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter.borrow_mut().peek().eq(&other.iter.borrow_mut().peek())
    }
}

impl<I, T> PartialOrd<Self> for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter.borrow_mut().peek().partial_cmp(&other.iter.borrow_mut().peek())
    }
}

impl<I, T> Ord for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter.borrow_mut().peek().cmp(&other.iter.borrow_mut().peek())
    }
}

impl<I, T> From<I> for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn from(iter: I) -> Self {
        Self { iter: RefCell::new(iter.peekable()) }
    }
}

impl<I, T> Deref for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    type Target = Peekable<I>;

    fn deref(&self) -> &Self::Target {
        let ptr = self.iter.as_ptr();
        unsafe { &*ptr }
    }
}

impl<I, T> DerefMut for IterHeapWrapper<I, T>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.iter.as_ptr();
        unsafe { &mut *ptr }
    }
}

fn main() {
    let a = [1, 2, 3];
    let b = [3, 2, 1];
    let mut heap: BinaryHeap<IterHeapWrapper<std::slice::Iter<'_, i32>, &i32>> = BinaryHeap::new();
    heap.push(IterHeapWrapper::from(a.iter()));
    heap.push(IterHeapWrapper::from(b.iter()));
    // assert_eq!(heap.pop().unwrap().next().unwrap(), &3);
    println!("{}", heap.pop().unwrap().next().unwrap());    // 3
}
*/