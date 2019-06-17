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
//
// Has commands for adding and loading clients, and returning to Main.
pub struct ClientMenuParser {
    clients: Vec<String>,
}
// specific implementation
impl ClientMenuParser {
    pub fn from(clients: Vec<String>) -> ClientMenuParser {
        ClientMenuParser {
            clients,
        }
    }

    fn load_client(&self, client_name: String) -> Command {
        if self.clients.contains(&client_name) {
            Command::Change(State::TaskMenu(client_name))
        } else {
            Command::Error(client_name)
        }
    }
}
// implementation of Parser
impl Parser for ClientMenuParser {
    fn parse(&self, s: &String) -> Command {
        let (first_word, rest_of_input) = split_at_first_space(s);
        match first_word.to_lowercase().as_str() {
            "hello"     => Command::Print("Hello from Client Menu!".to_string()),
            "quit"      => Command::Quit,
            "add"       => Command::AddClient(rest_of_input.to_string()),
            "load"      => self.load_client(rest_of_input.to_string()),
            "main"      => Command::Change(State::MainMenu),
            "back"      => Command::Change(State::MainMenu),
            _           => Command::Error((*s).clone()),
        }
    } 
}


// The Tasks menu.
//
// Has commands for adding tasks for a given client, and returning to Clients.
pub struct TaskMenuParser{
    client: String,
    tasks: Vec<String>,
}
// specific implementation
impl TaskMenuParser {
    // Constructor
    pub fn from(client: String, tasks: Vec<String>) -> TaskMenuParser {
        TaskMenuParser {
            client,
            tasks,
        }
    }
    // Loading task
    fn load_task(&self, task_name: String) -> Command {
        if self.tasks.contains(&task_name) {
            Command::Change(State::RecordMenu(self.client.clone(), task_name))
        } else {
            Command::Error(task_name)
        }
    }
}
// implementation of Parser
impl Parser for TaskMenuParser {
    fn parse(&self, s: &String) -> Command {
        let (first_word, rest_of_input) = split_at_first_space(s);
        match first_word.to_lowercase().as_str() {
            "hello"     => Command::Print(
                format!("Hello from {}'s Task Menu!", self.client)
            ),
            "quit"      => Command::Quit,
            "add"       => Command::AddTask(self.client.to_string(), rest_of_input.to_string()),
            "load"      => self.load_task(rest_of_input.to_string()),
            "back"      => Command::Change(State::ClientMenu),
            "main"      => Command::Change(State::MainMenu),
            _           => Command::Error((*s).clone()),
        }
    } 
}


pub struct RecordMenuParser {
    client: String,
    task: String,
    records: Vec<String>,
}
impl RecordMenuParser {
    pub fn from(client: String, task: String, records: Vec<String>) 
        -> RecordMenuParser {
        RecordMenuParser { client, task, records, }
    }
}
impl Parser for RecordMenuParser {
    fn parse(&self, s: &String) -> Command {
        let (first_word, rest_of_input) = split_at_first_space(s);
        match first_word.to_lowercase().as_str() {
            "hello" => Command::Print(
                format!(
                    "Hello from {}'s Record Menu for the {} Task",
                    self.client,
                    self.task
                )
            ),
            "quit" => Command::Quit,
            "back" => Command::Change(State::TaskMenu(self.client.clone())),
            _ => Command::Error((*s).clone()),
        }        
    }
}

