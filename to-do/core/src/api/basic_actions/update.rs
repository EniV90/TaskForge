use crate::structs::TodoItems;
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use to_do_dal::json_file::{get_all as get_all_handle, save_all};
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::update::UpdateOne;

pub async fn update<T: UpdateOne>(item: ToDoItem) -> Result<(), SchedulerServiceError> {
    let _ = T::update_one(item).await?;
    Ok(())
}
