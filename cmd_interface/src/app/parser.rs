use super::enums::{Command, State};


fn split_at_first_space(s: &String) -> (&str, &str) {
    match s.find(" ") {
        Some(i) => {
            (s[..i].trim(), s[i..].trim())
        },
        None => (s, "")
    }
}

pub trait Parser {
    fn parse(&self, s: &String) -> Command;
}


pub struct SplashPageParser;

impl Parser for SplashPageParser {
    fn parse(&self, _: &String) -> Command {
        Command::Change(State::MainMenu)
    }
}


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


pub struct ClientMenuParser {
    client_names: Vec<String>,
}
impl ClientMenuParser {
    pub fn from(client_names: Vec<String>) -> ClientMenuParser {
        ClientMenuParser {
            client_names: client_names,
        }
    }
}
impl Parser for ClientMenuParser {
    fn parse(&self, s: &String) -> Command {
        let (first_word, rest) = split_at_first_space(s);
        match first_word.to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Client Menu!".to_string()),
            "quit"      => Command::Quit,
            "tasks"     => Command::Change(State::TaskMenu("Test Client".to_string())),
            "load"      => Command::Change(State::TaskMenu(rest.to_string())),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error((*s).clone()),
        }
    } 
}


pub struct TaskMenuParser;

impl Parser for TaskMenuParser {
    fn parse(&self, s: &String) -> Command {
        match (*s).trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Task Menu!".to_string()),
            "quit"      => Command::Quit,
            "clients"   => Command::Change(State::ClientMenu),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error((*s).clone()),
        }
    } 
}