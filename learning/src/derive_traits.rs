// derive traits
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::fmt;

#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

impl fmt::Display for Inches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Inches", self.0)
    }
}

impl fmt::Display for Centimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Centimeters", self.0)
    }
}

#[derive(Debug)]
struct Seconds(i32);

impl PartialEq for Seconds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
// without impl PartialEq :     (rustc 1.70.0 2023-05-31)
        //->// error[E0369]: binary operation `==` cannot be applied to type `Seconds`
            //let this_is_true = one_second == one_second;
            // note: an implementation of `PartialEq<_>` might be missing for `Seconds`


impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Seconds", self.0)
    }
}

fn main() {
    let start = std::time::Instant::now();

    let one_second = Seconds(1);

    // Debug trait derive
    println!("One second looks like: {:?}", one_second);    // -> Seconds(1)
    
    // fmt::Display implement
    println!("One second looks like: {}", one_second);      // -> 1 Seconds

    let this_is_true = one_second == one_second;
    println!("Comparison result: {:?}", this_is_true);

    let foot = Inches(12);
    println!("One foot equals {}", foot);

    println!("One foot equals {}", foot.to_centimeters());

    let meter = Centimeters(100.0);
    println!("One meter equals {}", meter);


    // compare
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);

    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
