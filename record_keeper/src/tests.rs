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
