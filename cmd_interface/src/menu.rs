pub fn parse_command(command: String) -> bool {
    match command.trim().to_lowercase().as_str() {
        "hello"     => println!("Hello!"),
        "quit"      => return false,
        _ => println!("Command not recognized, please try again.")
    };
    true
}