pub fn parse_command(command: String) -> Option<String> {
    match command.trim().to_lowercase().as_str() {
        "hello"     => Some(hello()),
        "quit"      => return None,
        _           => Some(not_recognized(command)),
    }
}

fn hello() -> String {
    String::from("Hello!")
}

fn not_recognized(command: String) -> String {
    format!("Command '{}' not recognized, please try again", command)
}