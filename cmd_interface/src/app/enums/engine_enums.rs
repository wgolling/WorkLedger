pub enum State {
    SplashPage,
    MainMenu,
    ClientMenu,
    TaskMenu(String),
    RecordMenu(String, String),
}

pub enum Command {
    Quit,
    Change(State),
    Print(String),
    Error(String),
}
