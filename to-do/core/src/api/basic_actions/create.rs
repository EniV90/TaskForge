use crate::enums::TaskStatus;
use crate::structs::TodoItems;

use glue::errors::SchedulerServiceError;
#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

pub async fn create(item: TodoItems) -> Result<TodoItems, SchedulerServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;
    Ok(item)
}
