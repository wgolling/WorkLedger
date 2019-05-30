use std::process;

mod engine;
mod menu;
mod app;

fn main() {    
    // Run program.
    app::run();

    // Say goodbye / shudown program.
    println!("Quitting program.");
    process::exit(0);
}
