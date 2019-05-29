use record_keeper::RecordKeeper;

pub struct User {
    rk: RecordKeeper,
}

impl User {
    pub fn new() -> UserModel {
        UserModel {
            rk: RecordKeeper::new(),
        }
    }
}
