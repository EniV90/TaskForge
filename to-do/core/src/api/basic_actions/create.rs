use to_do_dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use to_do_dal::to_do_items::transactions::create::SaveOne;

use glue::errors::SchedulerServiceError;
#[cfg(feature = "json-file-storage")]

pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, SchedulerServiceError> {
    let created_item = T::save_one(item).await?;
    Ok(created_item)
}
