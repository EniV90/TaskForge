use std::fmt;

use crate::enums::TaskStatus;
use crate::structs::TodoItems;
use glue::errors::SchedulerServiceError;

#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

pub fn create(title: &str, status: TaskStatus) -> Result<TodoItems, SchedulerServiceError> {
    let items = TodoItems {
        title: title.to_string(),
        status,
    };

    let _ = save_one(&title.to_string(), &items)?;
    Ok(items)
}
