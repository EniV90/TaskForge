use crate::enums::TaskStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoItems {
    pub title: String,
    pub status: TaskStatus,
}

impl fmt::Display for TodoItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            TaskStatus::PENDING => write!(f, "Pending: {}", self.title),
            TaskStatus::DONE => write!(f, "Done: {}", self.title),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllTodoItems {
    pub done: Vec<TodoItems>,
    pub pending: Vec<TodoItems>,
}

impl AllTodoItems {
    pub fn from_hashmap(all_items: HashMap<String, TodoItems>) -> AllTodoItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for (_, items) in all_items {
            match items.status {
                TaskStatus::PENDING => pending.push(items),
                TaskStatus::DONE => done.push(items),
            }
        }
        AllTodoItems { done, pending }
    }
}
