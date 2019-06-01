use std::collections::HashMap;
use std::error::Error;

pub mod error;
pub use error::{ErrorType, DuplicateError, NotFoundError};


// public DataType enum 
// used for validating input
pub enum DataType {
    Client(String),
    Task(String),
}

// Task struct
struct Task {
    name: String,
}
impl Task {
    fn new(name: String) -> Task {
        Task { name }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

// Client struct
struct Client {
    name: String,
    tasks: HashMap<String, Task>,
}
impl Client {
    fn new(name: String) -> Client {
        Client {
            name,
            tasks: HashMap::new(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_task_names(&self) -> Vec<&String> {
        let mut result = Vec::new();
        for name in self.tasks.keys() {
            result.push(name);
        }
        result.sort();
        result
    }

    // Adding Tasks
    fn validate_task_name_for_add(&self, name: String) -> Option<DataType> {
        match self.tasks.get(&name) {
            Some(_) => None,
            None    => Some(DataType::Task(name)),
        }
    }
    fn add_validated_task(& mut self, task: DataType) -> Result<(), ErrorType> {
        match task {
            DataType::Task(name) => {
                self.tasks.insert(name.clone(), Task::new(name));
                Ok(())
            },
            _ => panic!("Client::add_validated_task got something that wasn't a task."),
        }
    }
    fn add_task(& mut self, name: String) -> Result<(), ErrorType> {
        match self.validate_task_name_for_add(name) {
            Some(data)  => {
                self.add_validated_task(data)
            },
            _ => Err(ErrorType::Duplicate(DuplicateError)),
        }
    }
}

// public RecordKeeper struct
pub struct RecordKeeper {
    clients: HashMap<String, Client>,
}
impl RecordKeeper {
    /// Returns a new RecordKeeper instance 
    pub fn new() -> RecordKeeper {
        RecordKeeper {
            clients: HashMap::new(),
        }
    }

    // Input validation methods
    fn validate_client_name_for_add(&self, name: String) -> Option<DataType> {
        match self.clients.get(&name) {
            Some(_) => None,
            None    => Some(DataType::Client(name)),
        }
    }
    fn validate_client_name_for_get(&self, name: String) -> Option<DataType> {
        match self.clients.get(&name) {
            Some(_) => Some(DataType::Client(name)),
            None    => None,
        }
    }

    // adding clients
    fn add_validated_client(& mut self, client: DataType) -> Result<(), ErrorType> {
        match client {
            DataType::Client(name) => {
                self.clients.insert(name.clone(), Client::new(name));
                Ok(())
            },
            _ => panic!("RecordKeeper::add_validated_client got something that wasn't a client."),
        }
    }
    /// Adds a new Client to the RecordKeeper
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the new client.
    ///
    /// # Remarks
    /// 
    /// Will throw an error if the client name is already in use.
    pub fn add_client(& mut self, name: String) -> Result<(), ErrorType> {
        match self.validate_client_name_for_add(name) {
            Some(data) => {
                self.add_validated_client(data)
            },
            _ => Err(ErrorType::Duplicate(DuplicateError)),
        }
    }

    // getting client names

    /// Returns a sorted list of the clients' names.
    pub fn get_client_names(&self) -> Vec<&String> {
        let mut result = Vec::new();
        for name in self.clients.keys() {
            result.push(name);
        }
        result.sort();
        result
    }

    /// Returns an owned vector of copies of clients' names.
    pub fn get_owned_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for name in self.clients.keys() {
            result.push((*name).clone());
        }
        result.sort();
        result
    }

    // adding tasks for a client
    fn add_task_for_validated_client(& mut self, client: DataType, task_name: String)
        -> Result<(), ErrorType> 
    {
        match client {
            DataType::Client(name) => 
                self
                .clients
                .get_mut(&name).unwrap()
                .add_task(task_name),
            _ => panic!("RecordKeeper::add_task_for_validated_client got something that wasn't a client."),            
        }
    }
    /// Adds a new Task to a client with the desired name
    /// 
    /// # Arguments
    ///
    /// * `client_name` - The name of the client the task
    /// is to be added to.
    /// * ` task_name` - The name of the new task.
    ///
    /// # Remarks
    /// 
    /// Will throw a NotFoundError if the client does not exist.
    /// Will throw a DuplicateError if task already exists for that client.
    pub fn add_task(& mut self, client_name: String, task_name: String)
        -> Result<(), ErrorType>
    {
        match self.validate_client_name_for_get(client_name) {
            Some(data) => self.add_task_for_validated_client(data, task_name),
            None       => Err(ErrorType::NotFound(NotFoundError)),
        }
    }

    // getting tasks for a client
    fn get_tasks_for_validated_client(&self, client: DataType) 
        -> Result<Vec<&String>, ErrorType> 
    {
        match client {
            DataType::Client(s) => Ok(self.clients.get(&s).unwrap().get_task_names()),
            _ => panic!("RecordKeeper::get_tasks_for_validated_client got something that wasn't a client."),
        }
    }
    /// Returns a sorted list of tasks for a given client, if it exists.
    pub fn get_tasks_for_client(&self, client: String) 
        -> Result<Vec<&String>, ErrorType> 
    {
        match self.validate_client_name_for_get(client) {
            Some(data) => self.get_tasks_for_validated_client(data),
            None       => Err(ErrorType::NotFound(NotFoundError)),
        }
    }
    /// Returns an owned sorted list of tasks for a given client, if it exists.
    pub fn get_owned_tasks_for_client(&self, client: String) 
        -> Result<Vec<String>, ErrorType> 
    {
        match self.get_tasks_for_client(client) {
            Ok(ref_vec) => Ok(RecordKeeper::ref_vec_to_owned(ref_vec)),
            Err(e)      => Err(e),
        }
    }

    fn ref_vec_to_owned(ref_vec: Vec<&String>) -> Vec<String> {
        ref_vec.into_iter().map( |name_ref| name_ref.clone() ).collect()
    }
}


#[cfg(test)]
mod tests;
