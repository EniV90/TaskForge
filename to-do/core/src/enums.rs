use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }
            &Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, SchedulerServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(SchedulerServiceError::new(
                "Invalid status".to_string(),
                SchedulerServiceErrorStatus::BadRequest,
            )),
        }
    }
}
