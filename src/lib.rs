mod error;
pub use error::AppError;

pub type QueryResult<T> = std::result::Result<T, AppError>;
pub type CommandResult = std::result::Result<(), AppError>;

#[derive(Clone, Debug)]
pub struct Task {
    id: u64,
    name: String,
}

impl Task {
    pub fn id(&self) -> u64 {
        self.id
    }
}

pub trait TaskRepository {
    fn list(&self) -> QueryResult<Vec<Task>>;
    fn find(&self, id: u64) -> QueryResult<&Task>;
    fn add(&mut self, task: Task) -> CommandResult;
    fn update(&mut self, task: Task) -> CommandResult;
    fn delete(&mut self, task: Task) -> CommandResult;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
