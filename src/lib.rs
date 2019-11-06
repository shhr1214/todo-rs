mod error;
pub use error::AppError;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type QueryResult<T> = std::result::Result<T, AppError>;
pub type CommandResult = std::result::Result<(), AppError>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    id: String,
    name: String,
}

impl Task {
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Task { id, name }
    }
}

pub trait TaskRepository {
    fn list(&self) -> QueryResult<Vec<Task>>;
    fn find(&self, id: String) -> QueryResult<&Task>;
    fn save(&mut self, task: Task) -> CommandResult;
    fn remove(&mut self, task: Task) -> CommandResult;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
