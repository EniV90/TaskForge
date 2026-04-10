use glue::errors::SchedulerServiceError;
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::update::UpdateOne;

pub async fn update<T: UpdateOne>(
    item: ToDoItem,
    user_id: i32,
) -> Result<(), SchedulerServiceError> {
    let _ = T::update_one(item, user_id).await?;
    Ok(())
}
