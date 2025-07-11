use std::error::Error;

pub fn handle_db_err<T>(result: Result<T, Box<dyn Error>>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(e) => {
            eprintln!("Database error: {}", e);
            None
        }
    }
}
