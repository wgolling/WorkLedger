pub enum State {
    SplashPage,
    MainMenu,
    ClientMenu,
    TaskMenu(String),
}

pub enum Command {
    Quit,
    Change(State),
    Print(String),
    Error(String),
}
