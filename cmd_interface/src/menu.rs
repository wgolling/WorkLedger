pub mod menu {
    pub enum Command {
        Quit,
        Change(MenuType),
        Print(String),
        Error,
    }

    pub enum MenuType {
        Main,
        Clients,
        Tasks,
        Sub,
    }

    pub trait MenuView {
        fn display(&self);
        fn parse_command(&self, command: String) -> Command;
    }

    pub struct Clients<'a> {
        clients: Vec<&'a str>,
    }
    impl<'a> Clients<'a> {
        pub fn new() -> Clients<'a> {
            Clients {
                clients: Vec::new(),
            }
        }
    }
    impl<'a> MenuView for Clients<'a> {
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

    pub struct Tasks<'a> {
        tasks: Vec<&'a str>,
    }
    impl<'a> Tasks<'a> {
        pub fn new() -> Tasks<'a> {
            Tasks {
                tasks: Vec::new(),
            }
        }
    }
    impl<'a> MenuView for Tasks<'a> {
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

    pub struct MainMenu {
        // dunno what it's going to have for fields yet.
    }
    impl MenuView for MainMenu {
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
        pub fn hello() -> String {
            String::from("Hello!")
        }
    }

    pub struct SubMenu {}
    impl MenuView for SubMenu {
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
}
