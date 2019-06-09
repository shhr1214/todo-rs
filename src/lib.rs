mod error;
pub use error::AppError;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Clone, Debug)]
pub struct Task {
    name: String,
}

pub trait TaskRepository {
    fn list(&self) -> Result<Vec<Task>>;
    fn find(&self, id: u64) -> Result<Task>;
    fn add(&self, task: Task) -> Result<()>;
    fn update(&self, task: Task) -> Result<()>;
    fn delete(&self, task: Task) -> Result<()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
