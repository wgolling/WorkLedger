pub struct Engine {
    menu: Box<Menu>,
}
impl Engine {
    pub fn new() -> Engine {
        Engine {
            menu: Box::new(MainMenu::new()),
        }
    }

    pub fn prompt(&self) {
        self.menu.prompt();
    }
    pub fn parse_command(&mut self, command: String) -> Option<String> {
        match (*self.menu).parse_command(command) {
            Command::Quit           => None,
            Command::Change(menu)   => {
                self.menu = menu;
                return Some(String::from("Changing menu."))
            }
            Command::Print(s)       => Some(s),
            Command::Error          => Some(String::from("Command not recognized.")),        
        }
    }
}

enum Command {
    Quit,
    Change(Box<Menu>),
    Print(String),
    Error,
}

trait Menu {
    fn prompt(&self);
    fn parse_command(&self, command: String) -> Command;
}

struct MainMenu {
    // dunno what it's going to have for fields yet.
}
impl Menu for MainMenu {
    fn prompt(&self) {
        println!("\nThis is the Main Menu.");
        println!("Please input a command.");
    }
    fn parse_command(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "hello"     => Command::Print(MainMenu::hello()),
            "quit"      => Command::Quit,
            "change"    => Command::Change(Box::new(SubMenu::new())),
            _           => Command::Error,
        }
    }    
}
impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {}
    }
    fn hello() -> String {
        String::from("Hello!")
    }

    fn not_recognized(command: String) -> String {
        format!("Command '{}' not recognized, please try again", command)
    }
}

struct SubMenu {}
impl Menu for SubMenu {
    fn prompt(&self) {
        println!("\nThis is the Sub-Menu.");
        println!("Please input a command.");
    }
    fn parse_command(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "quit"      => Command::Quit,
            "change"    => Command::Change(Box::new(MainMenu::new())),
            _           => Command::Error,
        }
    }    
}
impl SubMenu {
    pub fn new() -> SubMenu {
        SubMenu {}
    }
}
