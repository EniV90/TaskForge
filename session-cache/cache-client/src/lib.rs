use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use redis::Value;
use redis::aio::{ConnectionLike, MultiplexedConnection};
use std::error::Error;
#[derive(Debug)]
pub enum UserSessionStatus {
    Ok(i32),
    Refresh,
}

async fn get_connection(address: &str) -> Result<MultiplexedConnection, SchedulerServiceError> {
    let client = redis::Client::open(address).map_err(|e| {
        SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
    })?;
    let con = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
        })?;
    Ok(con)
}

fn unpack_result_string(result: Value) -> Result<String, SchedulerServiceError> {
    match result {
        Value::SimpleString(s) => Ok(s),
        Value::Okay => Ok("OK".to_string()),
        _ => Err(SchedulerServiceError::new(
            "Error converting result into string".to_string(),
            SchedulerServiceErrorStatus::Unknown,
        )),
    }
}

pub async fn login(
    address: &str,
    user_id: &str,
    timeout_mins: usize,
    perm_user_id: i32,
) -> Result<(), SchedulerServiceError> {
    let mut con = get_connection(address).await?;
    let result = con
        .req_packed_command(
            &redis::cmd("login.set")
                .arg(user_id)
                .arg(timeout_mins)
                .arg(perm_user_id.to_string())
                .clone(),
        )
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
        })?;
    match result {
        Value::Okay => return Ok(()),
        _ => {
            return Err(SchedulerServiceError::new(
                format!("{:?}", result),
                SchedulerServiceErrorStatus::Unknown,
            ));
        }
    }
}

pub async fn logout(address: &str, user_id: &str) -> Result<String, Box<dyn Error>> {
    let mut con = get_connection(address).await?;
    let result = con
        .req_packed_command(&redis::cmd("logout.set").arg(user_id).clone())
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
        })?;
    let result_string = unpack_result_string(result)?;
    Ok(result_string)
}

pub async fn update(
    address: &str,
    user_id: &str,
) -> Result<UserSessionStatus, SchedulerServiceError> {
    let mut con = get_connection(address).await?;
    let result = con
        .req_packed_command(&redis::cmd("updae.set").arg(user_id).clone())
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
        })?;
    let result_string = unpack_result_string(result)?;
    match result_string.as_str() {
        "TIMEOUT" => {
            return Err(SchedulerServiceError::new(
                "Session has times out".to_string(),
                SchedulerServiceErrorStatus::Unauthorized,
            ));
        }
        "NOT_FOUND" => {
            return Err(SchedulerServiceError::new(
                "Session not found".to_string(),
                SchedulerServiceErrorStatus::Unauthorized,
            ));
        }
        "REFRESH" => {
            return Ok(UserSessionStatus::Refresh);
        }
        _ => {}
    }
    let perm_user_id = match result_string.parse::<i32>() {
        Ok(perm_user_id) => perm_user_id,
        Err(_) => {
            return Err(SchedulerServiceError::new(
                "Error converting the result into string".to_string(),
                SchedulerServiceErrorStatus::Unknown,
            ));
        }
    };
    Ok(UserSessionStatus::Ok(perm_user_id))
}
