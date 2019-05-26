use std::fmt;
use std::error::Error;

pub struct DuplicateError;
// Implement std::fmt::Display for DuplicateError
impl fmt::Display for DuplicateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key already in use.") // user-facing output
    }
}
// Implement std::fmt::Debug for DuplicateError
impl fmt::Debug for DuplicateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
//Implement std::error::Error for DuplicateError
impl Error for DuplicateError {
    fn description(&self) -> &str {
        "Key already in use."
    }
    fn cause(&self) -> Option<&Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub struct NotFoundError;
// Implement std::fmt::Display for NotFoundError
impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key not found.") // user-facing output
    }
}
// Implement std::fmt::Debug for NotFoundError
impl fmt::Debug for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
//Implement std::error::Error for NotFoundError
impl Error for NotFoundError {
    fn description(&self) -> &str {
        "Key not found."
    }
    fn cause(&self) -> Option<&Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

