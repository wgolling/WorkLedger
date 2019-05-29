use super::enums::{Command, State};


pub trait Parser {
    fn parse(&self, s: String) -> Command;
}


pub struct SplashPageParser;

impl Parser for SplashPageParser {
    fn parse(&self, s: String) -> Command {
        Command::Change(State::MainMenu)
    }
}


pub struct MainMenuParser;

impl Parser for MainMenuParser {
    fn parse(&self, s: String) -> Command {
        match s.trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Main Menu!".to_string()),
            "quit"      => Command::Quit,
            "clients"   => Command::Change(State::ClientMenu),
            "tasks"     => Command::Change(State::TaskMenu),
            _           => Command::Error,
        }
    }
}


pub struct ClientMenuParser;

impl Parser for ClientMenuParser {
    fn parse(&self, s: String) -> Command {
        match s.trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Client Menu!".to_string()),
            "quit"      => Command::Quit,
            "tasks"      => Command::Change(State::TaskMenu),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error,
        }
    } 
}


pub struct TaskMenuParser;

impl Parser for TaskMenuParser {
    fn parse(&self, s: String) -> Command {
        match s.trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Task Menu!".to_string()),
            "quit"      => Command::Quit,
            "clients"   => Command::Change(State::ClientMenu),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error,
        }
    } 
}