use crate::structs::{AllTodoItems, TodoItems};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use to_do_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllTodoItems, SchedulerServiceError> {
    Ok(AllTodoItems::from_hashmap(get_all_handle::<TodoItems>()?))
}

pub async fn get_by_name(name: &str) -> Result<TodoItems, SchedulerServiceError> {
    Ok(get_all_handle::<TodoItems>()?
        .remove(name)
        .ok_or(SchedulerServiceError::new(
            format!("Items with name {} not found", name),
            SchedulerServiceErrorStatus::NotFound,
        ))?)
}
