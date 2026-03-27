use crate::structs::TodoItems;
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use to_do_dal::json_file::{get_all as get_all_handle, save_all};

pub async fn update(item: TodoItems) -> Result<(), SchedulerServiceError> {
    let mut all_items = get_all_handle::<TodoItems>()?;
    if !all_items.contains_key(&item.title) {
        return Err(SchedulerServiceError::new(
            format!("Item with name {} not found", item.title),
            SchedulerServiceErrorStatus::NotFound,
        ));
    }
    all_items.insert(item.title.clone(), item);
    save_all(&all_items)?;
    Ok(())
}
