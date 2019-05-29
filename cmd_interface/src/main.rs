use std::process;

mod engine;
mod menu;
mod app;

fn main() {    
    // // Display greeting message / spalsh page.
    // println!("Welcome to Will's Ledger Program.");
    
    // // Run main loop.
    // let mut me = engine::Engine::new();
    // me.run();

    app::run();
    
    // Say goodbye / shudown program.
    println!("Quitting program.");
    process::exit(0);
}
