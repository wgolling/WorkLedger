use std::collections::HashMap;
use std::error::Error;

pub mod error;
use error::{ErrorType, DuplicateError, NotFoundError};


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
        Task {
            name: name,
        }
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
            name: name,
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
}




#[cfg(test)]
mod tests {
    use super::*;

    // Testing Task
    #[test]
    fn make_task() {
        let task = Task::new(String::from("Test Task"));
        assert_eq!(task.get_name(), "Test Task");
    }

    //Testing Client
    #[test]
    fn make_client() {
        let client = Client::new(String::from("Test Client"));
        // a new Client should have a name and an empty hash table
        assert_eq!(client.get_name(), "Test Client");
        assert_eq!(client.get_task_names().len(), 0);
    }
    #[test]
    fn validating_task_names() {
        let mut client = Client::new(String::from("Test Client"));
        let test_name = String::from("Test Name");
        // If the task name doesn't exist yet, validate_task_name_for_add
        // will return data.
        match client.validate_task_name_for_add(test_name.clone()) {
            Some(DataType::Task(name)) => assert_eq!(*&name, "Test Name"),
            _ => assert!(false),
        } 
        let data = client.validate_task_name_for_add(test_name.clone()).unwrap();
        client.add_validated_task(data);
        // If task name already exists, returns None.
        match client.validate_task_name_for_add(test_name.clone()) {
            None => assert!(true),
            _    => assert!(false),
        }
    }
    #[test]
    #[should_panic]
    fn invalid_task_data() {
        let mut client = Client::new(String::from("Test Client"));
        let bad_data = DataType::Client(String::from("Test Client"));
        client.add_validated_task(bad_data);
    }
    #[test]
    fn add_task_to_client() {
        let mut client = Client::new(String::from("Test Client"));
        client.add_task(String::from("Test Task"));
        client.add_task(String::from("Test Task 2"));
        assert_eq!(client.get_task_names(), [ "Test Task", "Test Task 2" ]);
        // trying to add a duplicate name will result in an error
        assert!(client.add_task(String::from("Test Task")).is_err());
    }


    // Testing RecordKeeper
    #[test]
    fn make_new_record_keeper() {
        let rk = RecordKeeper::new();
        // A new RecordKeeper should have an empty HashMap.
        assert_eq!(rk.clients.len(), 0);
    }
    // input validation
    #[test]
    fn validating_client_names_for_add() {
        let mut rk = RecordKeeper::new();
        let test_name = String::from("Test Name");
        match rk.validate_client_name_for_add(test_name.clone()) {
            Some(DataType::Client(name)) => assert_eq!(*&name, "Test Name"),
            _ => assert!(false),
        }
        let data = rk.validate_client_name_for_add(test_name.clone());
        rk.add_validated_client(data.unwrap());
        assert_eq!(rk.clients.len(), 1);
    }
    fn validating_client_names_for_get() {
        let mut rk = RecordKeeper::new();
        let test_name = String::from("Test Name");
        match rk.validate_client_name_for_get(test_name.clone()) {
            None => assert!(true),
            _ => assert!(false),
        }
        rk.add_client(test_name.clone());
        match rk.validate_client_name_for_get(test_name.clone()) {
            Some(DataType::Client(name)) => assert_eq!(*&name, "Test Name"),
            _ => assert!(false),
        }        
    }
    #[test]
    #[should_panic]
    fn invalid_client_name_for_add() {
        let mut rk = RecordKeeper::new();
        let bad_data = DataType::Task(String::from("Test Task"));
        rk.add_validated_client(bad_data);
    }
    #[test]
    #[should_panic]
    fn invalid_client_name_for_get_tasks() {
        let mut rk = RecordKeeper::new();
        let bad_data = DataType::Task(String::from("Test Task"));
        rk.get_tasks_for_validated_client(bad_data);
    }
    #[test]
    #[should_panic]
    fn invalid_client_name_for_add_tasks() {
        let mut rk = RecordKeeper::new();
        let bad_data = DataType::Task(String::from("Test Task"));
        rk.add_task_for_validated_client(bad_data, String::from("Test Task"));
    }
    // adding clients
    #[test]
    fn add_clients() {
        let mut rk = RecordKeeper::new();
        rk.add_client(String::from("Test Client"));
        assert_eq!(rk.clients.len(), 1);
        assert!(rk.add_client(String::from("Test Client")).is_err());
    }
    // getting client names
    #[test]
    fn get_client_names() {
        let mut rk = RecordKeeper::new();
        rk.add_client(String::from("Test Client 2"));
        rk.add_client(String::from("Test Client"));
        let list = rk.get_client_names();
        assert_eq!(list, [ "Test Client", "Test Client 2" ]);
    }
    // adding tasks for a client
    #[test]
    fn add_tasks() {
        let mut rk = RecordKeeper::new();
        let test_client = String::from("Test Client");
        let test_task   = String::from("Test Task");
        // should throw NotFoundError
        match rk.add_task(test_client.clone(), test_task.clone()) {
            Err(ErrorType::NotFound(_)) => assert!(true),
            _                           => assert!(false),
        };
        rk.add_client(test_client.clone());
        rk.add_client(String::from("Test Client 2"));
        rk.add_task(test_client.clone(), test_task.clone());
        // should throw DuplicateError
        match rk.add_task(test_client.clone(), test_task.clone()) {
            Err(ErrorType::Duplicate(_)) => assert!(true),
            _                            => assert!(false),
        };
        rk.add_task(String::from("Test Client 2"), test_task.clone());
        rk.add_task(test_client.clone(), String::from("Test Task 2"));
        assert_eq!(rk.get_client_names(), [ "Test Client", "Test Client 2" ]);
        assert!(rk.add_task(String::from("Test Client 2"), test_task.clone()).is_err());
    }
    // getting tasks for a client
    #[test]
    fn get_client_tasks() {
        let mut rk = RecordKeeper::new();
        rk.add_client(String::from("Test Client"));
        rk.add_task(String::from("Test Client"), String::from("Test Task"));
        rk.add_task(String::from("Test Client"), String::from("Test Task 2"));
        match rk.get_tasks_for_client(String::from("Test Client")) {
            Ok(vector) => assert_eq!(vector, ["Test Task", "Test Task 2"]),
            _ => assert!(false),
        }
        assert!(rk.get_tasks_for_client(String::from("Bad String")).is_err());   
    }
}







