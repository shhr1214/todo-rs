//! # InMemory な TaskRepository の実装
//!
//! メモリ上にデータを保存する。persistence って名前だけど永続化しない。
//! 主にテスト用途。

use std::collections::HashMap;

use todo_rs::{AppError, CommandResult, QueryResult, Task, TaskRepository};

/// InMmeory な TaskRepository を実装した構造体。
pub struct InMemoryTaskRepository {
    /// タスクは HashMap で持つ。キーは ID (u64)。
    tasks: HashMap<u64, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> InMemoryTaskRepository {
        InMemoryTaskRepository {
            tasks: HashMap::new(),
        }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn list(&self) -> QueryResult<Vec<Task>> {
        Ok(self.tasks.iter().map(|(_, v)| v.clone()).collect())
    }

    fn find(&self, id: u64) -> QueryResult<&Task> {
        match self.tasks.get(&id) {
            Some(task) => Ok(task),
            _ => Err(AppError::NotFound),
        }
    }

    fn add(&mut self, task: Task) -> CommandResult {
        let id = task.id();
        if self.tasks.contains_key(&id) {
            return Err(AppError::AlreadyExists(id));
        }

        self.tasks.insert(task.id(), task);
        Ok(())
    }

    fn update(&mut self, task: Task) -> CommandResult {
        let id = task.id();
        if self.tasks.contains_key(&id) {
            return Err(AppError::AlreadyExists(id));
        }

        self.tasks.insert(task.id(), task);
        Ok(())
    }

    fn delete(&mut self, task: Task) -> CommandResult {
        let id = task.id();
        if !self.tasks.contains_key(&id) {
            return Err(AppError::NotFound);
        }

        self.tasks.remove(&task.id());
        Ok(())
    }
}
