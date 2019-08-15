use std::process;
use std::time::{Duration, Instant};

mod app;

fn main() {  
    // Start timer.  
    let start = Instant::now();

    // Run program.
    app::run();

    // Split timer.
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    // Say goodbye / shudown program.
    println!("Quitting program.");
    process::exit(0);
}
