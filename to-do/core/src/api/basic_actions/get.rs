use crate::structs::{AllTodoItems, TodoItems};
use glue::errors::SchedulerServiceError;
use to_do_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllTodoItems, SchedulerServiceError> {
    Ok(AllTodoItems::from_hashmap(get_all_handle::<TodoItems>()?))
}
