use std::process;

mod engine;
mod menu;

fn main() {
    //record_keeper::print_hello();
    
    // Display greeting message / spalsh page.
    println!("Welcome to Will's Ledger Program.");
    
    // Run main loop.
    let mut me = engine::Engine::new();
    me.run();


    // Say goodbye / shudown program.
    println!("Quitting program.");
    process::exit(0);
}
