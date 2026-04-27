use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::ContentType;
use actix_web::test::init_service;
use actix_web::{App, HttpRequest, HttpResponse, web};
use auth_kernel::user_sessions::transactions::get::GetUserSession;
use auth_kernel::{api::users::get::get_user_by_unique_id, user_sessions::schema::UserSession};
use glue::{
    errors::{SchedulerServiceError, SchedulerServiceErrorStatus},
    token::HeaderToken,
};
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::schema::AllToDOItems;
use to_do_dal::to_do_items::{
    schema::ToDoItem,
    transactions::{
        delete::{DeleteOne, DeleteOneResponse},
        get::GetAll,
    },
};

pub async fn delete_by_name<T, X>(
    token: HeaderToken,
    req: HttpRequest,
) -> Result<HttpResponse, SchedulerServiceError>
where
    T: DeleteOne + GetAll,
    X: GetUserSession,
{
    let session = X::get_user_session(token.unique_id).await?;

    // let user = get_user_by_unique_id(token.unique_id).await?;
    match req.match_info().get("name") {
        Some(name) => {
            delete_core::<T>(name, session.user_id).await?;
        }
        None => {
            return Err(SchedulerServiceError::new(
                "Name not found".to_string(),
                SchedulerServiceErrorStatus::BadRequest,
            ));
        }
    }
    Ok(HttpResponse::Ok().json(get_all_core::<T>(session.user_id).await?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::Request;
    use actix_web::{
        self, App,
        body::MessageBody,
        dev::ServiceResponse,
        http::header::ContentType,
        test::{TestRequest, call_service, init_service},
        web,
    };
    use auth_kernel::user_sessions::schema::UserSession;
    use glue::token::HeaderToken;
    use std::future::Future;
    use to_do_dal::to_do_items::schema::{AllToDOItems, ToDoItem};
    use to_do_dal::to_do_items::transactions::delete::DeleteOneResponse;

    // Function that generates an item
    fn generate_to_do_item() -> ToDoItem {
        ToDoItem {
            id: 1,
            title: "test".to_string(),
            status: "PENDING".to_string(),
        }
    }

    // Function that generates the return items
    fn generate_get_all_return() -> Vec<ToDoItem> {
        vec![generate_to_do_item()]
    }

    // Mock DB Handle
    struct MockDbHandle;

    impl DeleteOne for MockDbHandle {
        fn delete_one(
            title: String,
            _user_id: i32,
        ) -> impl Future<Output = DeleteOneResponse> + Send {
            async fn run(title: String) -> DeleteOneResponse {
                if title == "coding" {
                    return Ok(generate_to_do_item());
                }
                return Err(SchedulerServiceError::new(
                    "Item not found".to_string(),
                    SchedulerServiceErrorStatus::NotFound,
                ));
            }
            run(title)
        }
    }

    impl GetAll for MockDbHandle {
        fn get_all(
            user_id: i32,
        ) -> impl Future<Output = Result<Vec<ToDoItem>, SchedulerServiceError>> + Send {
            async fn run(user_id: i32) -> Result<Vec<ToDoItem>, SchedulerServiceError> {
                if user_id == 2 {
                    return Err(SchedulerServiceError::new(
                        "error getting items".to_string(),
                        SchedulerServiceErrorStatus::Unknown,
                    ));
                }
                Ok(generate_get_all_return())
            }
            run(user_id)
        }
    }

    // Mock User Cache Handle
    struct MockUserSessionHandle;

    impl GetUserSession for MockUserSessionHandle {
        fn get_user_session(
            unique_id: String,
        ) -> impl Future<Output = Result<UserSession, SchedulerServiceError>> {
            async fn run(unique_id: String) -> Result<UserSession, SchedulerServiceError> {
                if unique_id == "break" {
                    return Err(SchedulerServiceError::new(
                        "User not found".to_string(),
                        SchedulerServiceErrorStatus::NotFound,
                    ));
                }

                if unique_id == "2" {
                    return Ok(UserSession { user_id: 2 });
                }
                Ok(UserSession { user_id: 1 })
            }
            run(unique_id)
        }
    }

    // Define Mock  Service
    async fn run_request(req: Request) -> ServiceResponse {
        let service = delete_by_name::<MockDbHandle, MockUserSessionHandle>;

        let app = init_service(App::new().route("/delete/{name}", web::delete().to(service))).await;
        call_service(&app, req).await
    }

    #[tokio::test]
    async fn test_delete_ok() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "test_id".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/coding")
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();
        let body: AllToDOItems = serde_json::from_str(body_str).unwrap();

        assert_eq!(status, 200);
        assert_eq!(
            body,
            AllToDOItems::from_vec(generate_get_all_return()).unwrap()
        );
    }

    // Test Invalid Token
    #[tokio::test]
    async fn test_delete_invalid_token() {
        // Configure service
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }
        // Make request and get response
        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header(("token", "test"))
            .uri("/delete/coding")
            .to_request();
        let resp = run_request(req).await;

        // extract status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 401);
        assert_eq!(body_str, "\"token not a valid string\"");
    }

    // Test User not found
    #[tokio::test]
    async fn test_delete_user_not_found() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        // Make request and get response
        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "break".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/coding")
            .to_request();
        let resp = run_request(req).await;

        // extract status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 404);
        assert_eq!(body_str, "\"User not found\"");
    }

    #[tokio::test]
    async fn test_delete_item_not_found() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "test_id".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/break")
            .to_request();
        let resp = run_request(req).await;

        // extract status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 404);
        assert_eq!(body_str, "\"Item not found\"");
    }

    #[tokio::test]
    async fn test_delete_get_all_error() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        let req = TestRequest::delete()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken {
                    unique_id: "2".to_string(),
                }
                .encode()
                .unwrap(),
            ))
            .uri("/delete/coding")
            .to_request();

        let resp = run_request(req).await;

        // extract status and body
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // assert the status and body
        assert_eq!(status, 500);
        assert_eq!(body_str, "\"error getting items\"");
    }
}
