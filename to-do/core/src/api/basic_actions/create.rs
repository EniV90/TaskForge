use crate::enums::TaskStatus;
use crate::structs::TodoItems;

use to_do_dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use to_do_dal::to_do_items::transactions::create::SaveOne;

use glue::errors::SchedulerServiceError;
#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, SchedulerServiceError> {
    let created_item = T::save_one(item).await?;
    Ok(created_item)
}
