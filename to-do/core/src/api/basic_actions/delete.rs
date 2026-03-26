use crate::structs::TodoItems;
use glue::errors::SchedulerServiceError;
use to_do_dal::json_file::delete_one;

pub async fn delete(id: &str) -> Result<TodoItems, SchedulerServiceError> {
    delete_one::<TodoItems>(id)
}
