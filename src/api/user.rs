use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;

#[macro_export]
macro_rules! find_all {
($db:expr) => {
    self::routes::user_route::find_all().and(with_db($db)).and_then(self::handlers::user_handler::find_all)
}
}

#[macro_export]
macro_rules! find_by_id {
($db:expr) => {
    self::routes::user_route::find_by_id().and(with_db($db)).and_then(self::handlers::user_handler::find_by_id)
}
}

#[macro_export]
macro_rules! create {
($db:expr,  $sse:expr) => {
    self::routes::user_route::create().and(with_db($db)).and(with_sse($sse)).and_then(self::handlers::user_handler::create)
}
}

#[macro_export]
macro_rules! update {
($db:expr,  $sse:expr) => {
    self::routes::user_route::update().and(with_db($db)).and(with_sse($sse)).and_then(self::handlers::user_handler::update)
}
}

#[macro_export]
macro_rules! delete {
($db:expr,  $sse:expr) => {
    self::routes::user_route::delete().and(with_db($db)).and(with_sse($sse)).and_then(self::handlers::user_handler::delete)
}
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub age: u8,
}

impl UserRequest {
    pub fn new(name: String, age: u8) -> Self {
        UserRequest {
            name,
            age,
        }
    }
}

impl From<UserRequest> for User {
    fn from(ur: UserRequest) -> Self {
        User::new(ur.name, ur.age)
    }
}

#[derive(Serialize)]
pub struct UserResponse<'a> {
    pub id: &'a Uuid,
    pub name: &'a String,
    pub age: &'a u8,
}

impl<'a> UserResponse<'a> {
    pub fn new(id: &'a Uuid, name: &'a String, age: &'a u8) -> Self {
        UserResponse {
            id,
            name,
            age,
        }
    }
}

impl<'a> From<(&'a Uuid, &'a User)> for UserResponse<'a> {
    fn from(t: (&'a Uuid, &'a User)) -> Self {
        UserResponse::new(t.0, &t.1.name, &t.1.age)
    }
}
