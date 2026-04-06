use super::enums::TaskStatus;
use glue::errors::SchedulerServiceError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: i32,
    pub title: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllToDOItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllToDOItems {
    pub fn from_vec(all_items: Vec<ToDoItem>) -> Result<AllToDOItems, SchedulerServiceError> {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items {
            match TaskStatus::from_string(&item.status)? {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item),
            }
        }
        Ok(AllToDOItems { pending, done })
    }
}
