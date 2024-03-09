// https://github.com/actix/actix-web/issues/1087
// https://docs.rs/actix-web/latest/actix_web/middleware/struct.DefaultHeaders.html

#[allow(unused_imports)]
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
//use futures::{future::ok, stream::once}; // streaming

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
    age: i32,
}

fn sum_array(nums: &[i64]) -> i64 {
    nums.iter().sum()
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/")]
async fn hello() -> &'static str {
    let start = std::time::Instant::now();
    let duration: std::time::Duration = start.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n💖 Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    "💖 hello world!"
}

#[get("/iterator")]
async fn iterator() -> impl Responder {
    let start = std::time::Instant::now();

    let mut sum: i64 = 0;
    for i in 1..=1_000_000 {
        sum += i;
    }

    let duration: std::time::Duration = start.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n💖 Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    let response = HttpResponse::Ok()
        .content_type("application/json")
        // .insert_header(("Server", "Mac Pro"))
        // .insert_header(("X-Target-OS", std::env::consts::OS))
        // // .insert_header(("X-Actix-Version", env!("CARGO_PKG_VERSION")))
        // .insert_header(("X-Powered-By", "Actix Engine"))
        // .insert_header(("Developed-By", "Nguyen Huu Anh Tuan"))
        .body(sum.to_string());

    response
}

#[get("/json")]
async fn serde_json_map() -> impl Responder {
    let start = std::time::Instant::now();

    let mut query_result = vec![
        // mutable so that we can sort
        User {
            name: "Alex".to_string(),
            age: 12,
        },
        User {
            name: "John".to_string(),
            age: 35,
        },
        User {
            name: "Nguyễn Hữu Anh Tuấn".to_string(),
            age: 19,
        },
        User {
            name: "Bob".to_string(),
            age: 45,
        },
    ];
    // sorting:
    query_result.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    let transformed_result_as_boxed_slice: Vec<String> = query_result
        .iter()
        .filter(|user| user.age > 10)
        .map(|user| format!("Name: 💖{}, Age: {}", user.name, user.age))
        .collect();

    // color printing causes 3 microsecond overhead
    // Create a StandardStream for writing colored output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Set the color specification for the text
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Rgb(235, 137, 52)));
    // Write the colored text to stdout
    stdout.set_color(&color_spec).unwrap();

    let duration: std::time::Duration = start.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n💖 Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    // Reset the text color back to the default
    stdout.reset().unwrap();

    //HttpResponse::Ok().body(transformed_result_as_boxed_slice)
    let body = serde_json::to_string(&transformed_result_as_boxed_slice).unwrap();

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        // .insert_header(("Server", "Mac Pro"))
        // .insert_header(("X-Target-OS", std::env::consts::OS))
        // // .insert_header(("X-Actix-Version", env!("CARGO_PKG_VERSION")))
        // .insert_header(("X-Powered-By", "Actix Engine"))
        // .insert_header(("Developed-By", "Nguyen Huu Anh Tuan"))
        .body(body)

}

#[get("/sum-array")]
async fn calc_sum_array() -> impl Responder {
    let nums: Vec<i64> = (1..=1000000).collect();
    let start_time = std::time::Instant::now();
    let result = sum_array(&nums);

    println!("Sum Array Result: {}", result);
    // end of main
    let duration: std::time::Duration = start_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n💖 Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    HttpResponse::Ok().body(result.to_string())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            // .wrap_fn(|req, srv| {
            //     let resp = srv.call(req);
            //     let mut new_resp = HttpResponse::from_resp(resp);
            //     // Add custom response headers
            //     new_resp
            //         .headers_mut()
            //         .insert("X-Custom-Header", "Hello from Middleware");
            //     new_resp
            // })
            .wrap(middleware::DefaultHeaders::new()
                .add(("X-Version", "0.2"))
                .add(("Server", "Mac Pro"))
                .add(("X-Target-OS", std::env::consts::OS))
                // .add(("X-Actix-Version", env!("CARGO_PKG_VERSION")))
                .add(("X-Powered-By", "Actix Engine"))
                .add(("Developed-By", "Nguyen Huu Anh Tuan"))
            )

            .service(hello)
            // if not found
            .default_service(web::to(|| async { HttpResponse::Ok().content_type("application/json; charset=utf-8").body("I feel lost 🧐") }))
            .service(health_checker_handler)
            .service(echo)
            .service(iterator)
            .service(serde_json_map)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/algorithms").service(calc_sum_array))
    })
    .bind(("127.0.0.1", 8080))
    {
        Ok(server) => {
            println!("🚀 Server started successfully at http://127.0.0.1:8080");
            server.run().await
        }
        Err(e) => {
            eprint!("Error starting server: {}", e);
            Err(e)
        }
    }
}
