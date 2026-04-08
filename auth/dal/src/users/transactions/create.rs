use crate::users::schema::{NewUser, User};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use super::super::descriptors::SqlxPostGresDescriptor;