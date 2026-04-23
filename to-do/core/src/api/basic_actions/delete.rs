use glue::errors::SchedulerServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(id: &str, user_id: i32) -> Result<(), SchedulerServiceError> {
    let _ = T::delete_one(id.to_string(), user_id).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::future::Future;
    use to_do_dal::to_do_items::schema::ToDoItem;
    use to_do_dal::to_do_items::transactions::delete::DeleteOneResponse;

    #[tokio::test]
    async fn test_delete_ok() {
        struct ReturnOneMock;

        impl DeleteOne for ReturnOneMock {
            fn delete_one(
                id: String,
                user_id: i32,
            ) -> impl Future<Output = DeleteOneResponse> + Send {
                if id != "coding" {
                    panic!("Invalid title")
                }
                if user_id != 1 {
                    panic!("Invalid user_id")
                }
                async {
                    Ok(ToDoItem {
                        id: 1,
                        title: "title".to_string(),
                        status: "status".to_string(),
                    })
                }
            }
        }
        let result = delete::<ReturnOneMock>("coding", 1).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_delete_err() {
        struct ReturnErorMock;

        impl DeleteOne for ReturnErorMock {
            fn delete_one(
                id: String,
                user_id: i32,
            ) -> impl Future<Output = DeleteOneResponse> + Send {
                if id != "coding" {
                    panic!("Invalid Title");
                }
                if user_id != 1 {
                    panic!("Invalid user_id");
                }
                async {
                    Err(SchedulerServiceError::new(
                        "Some Error".to_string(),
                        glue::errors::SchedulerServiceErrorStatus::NotFound,
                    ))
                }
            }
        }
        let result = delete::<ReturnErorMock>("coding", 1).await;
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => {
                assert_eq!(
                    e.status,
                    glue::errors::SchedulerServiceErrorStatus::NotFound
                );
                assert_eq!(e.message, "Some Error")
            }
        }
    }
}
