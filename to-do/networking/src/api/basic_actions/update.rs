use actix_web::{HttpResponse, web::Json};
use glue::{errors::SchedulerServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{
    create::create as create_core, get::get_all as get_all_core, update::update as update_core,
};
use to_do_core::structs::TodoItems;

pub async fn update(
    token: HeaderToken,
    body: Json<TodoItems>,
) -> Result<HttpResponse, SchedulerServiceError> {
    println!("Token: {}", token.message);
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
