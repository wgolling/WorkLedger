use super::enums::{Command, State};

// Helper function for formatting input.
fn split_at_first_space(s: &String) -> (&str, &str) {
    match s.find(" ") {
        Some(i) => {
            (s[..i].trim(), s[i..].trim())
        },
        None => (s, "")
    }
}


// Wraps a single parse method that returns a Command variant.
pub trait Parser {
    fn parse(&self, s: &String) -> Command;
}


// The entry point of the program. Every command goes to the Main menu.
pub struct SplashPageParser;

impl Parser for SplashPageParser {
    fn parse(&self, _: &String) -> Command {
        Command::Change(State::MainMenu)
    }
}


// The Main menu.
// Has commands for navigating to submenus.
pub struct MainMenuParser;

impl Parser for MainMenuParser {
    fn parse(&self, s: &String) -> Command {
        match (*s).trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Main Menu!".to_string()),
            "quit"      => Command::Quit,
            "clients"   => Command::Change(State::ClientMenu),
            "tasks"     => Command::Change(State::TaskMenu("Test Client".to_string())),
            _           => Command::Error((*s).clone()),
        }
    }
}


// The Clients menu.
// Has commands for adding and loading clients, and returning to Main.
pub struct ClientMenuParser;

impl Parser for ClientMenuParser {
    fn parse(&self, s: &String) -> Command {
        let (first_word, rest_of_input) = split_at_first_space(s);
        match first_word.to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Client Menu!".to_string()),
            "quit"      => Command::Quit,
            "tasks"     => Command::Change(State::TaskMenu("Test Client".to_string())),
            "load"      => Command::Change(State::TaskMenu(rest_of_input.to_string())),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error((*s).clone()),
        }
    } 
}


// The Tasks menu.
// Has commands for adding tasks for a given client, and returning to Clients.
pub struct TaskMenuParser;

impl Parser for TaskMenuParser {
    fn parse(&self, s: &String) -> Command {
        match (*s).trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Task Menu!".to_string()),
            "quit"      => Command::Quit,
            "back"      => Command::Change(State::ClientMenu),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error((*s).clone()),
        }
    } 
}