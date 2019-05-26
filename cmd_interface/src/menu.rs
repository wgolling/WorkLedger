use record_keeper::RecordKeeper;


// Idea: Give Engine a generic type parameter to avoid lifetime parameters.
//       Make RecordKeeper trait, and implement JTRecordKeeper.
pub struct Engine {
    menu: Box<Menu>,
    rk: RecordKeeper,
}
impl Engine {
    pub fn new() -> Engine {
        Engine {
            menu: Box::new(MainMenu::new()),
            rk: RecordKeeper::new(),
        }
    }
    pub fn from(rk: RecordKeeper) -> Engine {
        Engine {
            menu: Box::new(MainMenu::new()),
            rk: rk,
        }
    }

    pub fn display(&self) {
        self.menu.display();
    }
    pub fn parse_command(&mut self, command: String) -> Option<String> {
        match (*self.menu).parse_command(command) {
            Command::Quit     => None,
            Command::Change(menu_type)   
                              => {
                self.change_menu(menu_type);
                return Some(String::from("Changing menu."))
            }
            Command::Print(s) => Some(s),
            Command::Error    => Some(String::from("Command not recognized.")),        
        }
    }
    fn change_menu(& mut self, menu: MenuType) {
        self.menu = match menu {
            MenuType::Main    => Box::new(MainMenu::new()),
            MenuType::Clients => Box::new(Clients::new()),
            MenuType::Tasks   => Box::new(Tasks::new()),
            MenuType::Sub     => Box::new(SubMenu::new()),
        };
    }
}

enum Command {
    Quit,
    Change(MenuType),
    Print(String),
    Error,
}

enum MenuType {
    Main,
    Clients,
    Tasks,
    Sub,
}

trait Menu {
    fn display(&self);
    fn parse_command(&self, command: String) -> Command;
}

struct Clients<'a> {
    clients: Vec<&'a str>,
}
impl<'a> Clients<'a> {
    pub fn new() -> Clients<'a> {
        Clients {
            clients: Vec::new(),
        }
    }
}
impl<'a> Menu for Clients<'a> {
    fn display(&self) {
        println!("\nThis is the Clients Menu.");
        println!("Please select one of the following clients:");
    }
    fn parse_command(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "hello"     => Command::Print(MainMenu::hello()),
            "quit"      => Command::Quit,
            "change"    => Command::Change(MenuType::Sub),
            _           => Command::Error,
        }
    }        
}

struct Tasks<'a> {
    tasks: Vec<&'a str>,
}
impl<'a> Tasks<'a> {
    pub fn new() -> Tasks<'a> {
        Tasks {
            tasks: Vec::new(),
        }
    }
}
impl<'a> Menu for Tasks<'a> {
    fn display(&self) {
        println!("\nThis is the Tasks Menu.");
        println!("Please select one of the following tasks:");
    }
    fn parse_command(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "hello"     => Command::Print(MainMenu::hello()),
            "quit"      => Command::Quit,
            "change"    => Command::Change(MenuType::Sub),
            _           => Command::Error,
        }
    }        
}

struct MainMenu {
    // dunno what it's going to have for fields yet.
}
impl Menu for MainMenu {
    fn display(&self) {
        println!("\nThis is the Main Menu.");
        println!("Please input a command.");
    }
    fn parse_command(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "hello"     => Command::Print(MainMenu::hello()),
            "quit"      => Command::Quit,
            "change"    => Command::Change(MenuType::Sub),
            "clients"   => Command::Change(MenuType::Clients),
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
}

struct SubMenu {}
impl Menu for SubMenu {
    fn display(&self) {
        println!("\nThis is the Sub-Menu.");
        println!("Please input a command.");
    }
    fn parse_command(&self, command: String) -> Command {
        match command.trim().to_lowercase().as_str() {
            "quit"      => Command::Quit,
            "change"    => Command::Change(MenuType::Main),
            _           => Command::Error,
        }
    }    
}
impl SubMenu {
    pub fn new() -> SubMenu {
        SubMenu {}
    }
}
