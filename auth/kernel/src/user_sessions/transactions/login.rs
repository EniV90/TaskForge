use crate::user_sessions::descriptors::RedisSessionDescriptor;
use cache_client::login as cache_login;
use glue::errors::SchedulerServiceError;
use std::future::Future;

pub trait LoginUserSession {
    fn login_user_session(
        address: &str,
        user_id: &str,
        timeout_mins: usize,
        perm_user_id: i32,
    ) -> impl Future<Output = Result<(), SchedulerServiceError>>;
}
impl LoginUserSession for RedisSessionDescriptor {
    fn login_user_session(
        address: &str,
        user_id: &str,
        timeout_mins: usize,
        perm_user_id: i32,
    ) -> impl Future<Output = Result<(), SchedulerServiceError>> {
        cache_login(address, user_id, timeout_mins, perm_user_id)
    }
}
