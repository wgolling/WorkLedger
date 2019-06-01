use record_keeper::RecordKeeper;
pub use record_keeper::{ErrorType, DuplicateError, NotFoundError};


// A model which wraps a RecordKeeper instance.
pub struct User {
    rk: RecordKeeper,
}

impl User {
    pub fn new() -> User {
        User {
            rk: RecordKeeper::new(),
        }
    }

    // Adding clients.
    pub fn add_client(& mut self, name: String) -> Result<(), ErrorType> {
        self.rk.add_client(name)
    }

    // Getting client names.
    pub fn get_client_names(&self) -> Vec<&String> {
        self.rk.get_client_names()
    }
    pub fn get_owned_names(&self) -> Vec<String> {
        self.rk.get_owned_names()
    }

    // Addings tasks for clients.
    pub fn add_task(& mut self, client_name: String, task_name: String)
        -> Result<(), ErrorType>
    {
        self.rk.add_task(client_name, task_name)
    }

    // Getting task names for clients.
    pub fn get_owned_tasks_for_client(&self, client_name: String) 
        -> Result<Vec<String>, ErrorType> 
    {
        self.rk.get_owned_tasks_for_client(client_name)
    }
}
