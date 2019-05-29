use super::enums::{Command, State};


pub trait Parser {
    fn parse(&self, s: String) -> Command;
}


pub struct MainMenuParser;

impl Parser for MainMenuParser {
    fn parse(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "hello"     => Command::Print("Hello!".to_string()),
            "quit"      => Command::Quit,
            "change"    => Command::Change(State::SplashPage),
            "clients"   => Command::Change(State::ClientMenu),
            _           => Command::Error,
        }
    }
}