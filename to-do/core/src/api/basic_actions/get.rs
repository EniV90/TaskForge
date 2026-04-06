use crate::structs::TodoItems;
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use to_do_dal::json_file::get_all as get_all_handle;
use to_do_dal::to_do_items::schema::AllToDOItems;
use to_do_dal::to_do_items::transactions::get::GetAll;

pub async fn get_all<T: GetAll>() -> Result<AllToDOItems, SchedulerServiceError> {
    let all_items = T::get_all().await?;
    AllToDOItems::from_vec(all_items)
}

pub async fn get_by_name(name: &str) -> Result<TodoItems, SchedulerServiceError> {
    Ok(get_all_handle::<TodoItems>()?
        .remove(name)
        .ok_or(SchedulerServiceError::new(
            format!("Items with name {} not found", name),
            SchedulerServiceErrorStatus::NotFound,
        ))?)
}
