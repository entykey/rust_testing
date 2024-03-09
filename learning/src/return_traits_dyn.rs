// Returning Traits with dyn
use rand::Rng;

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal() -> Box<dyn Animal> {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen(); // Generate a random f64 between 0 and 1

    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let start = std::time::Instant::now();

    
    let animal = random_animal();
    println!("You've randomly chosen an animal, and it says {}", animal.noise());


    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
