use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status};

use crate::api::sse::Message;
use crate::api::user::{UserRequest, UserResponse};
use crate::models::user::User;
use crate::emitter::sse_emitter::Sse;
use crate::emitter::sse_emitter::publish;
use crate::repository::user_repository::Database;

pub async fn find_all(db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let repository = db.read().await;
    let users = repository.find_all()
        .iter()
        .map(|v| {
            UserResponse::from(v)
        }).collect::<Vec<UserResponse>>();

    Ok(warp::reply::json(&users))
}

pub async fn find_by_id(id: Uuid, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let repository = db.read().await;

    match repository.find_by_id(&id) {
        None => Ok(with_status(json(&format!("User id: {} not found", &id)), StatusCode::NOT_FOUND)),//issue https://github.com/seanmonstar/warp/issues/77
        Some(u) => Ok(with_status(json(&UserResponse::new(&id, &u.name, &u.age)), StatusCode::OK))
    }
}

pub async fn create(user: UserRequest, db: Database, sse: Sse) -> Result<impl warp::Reply, warp::Rejection> {
    let id = Uuid::new_v4();
    let user: User = user.into();

    let mut repository = db.write().await;
    repository.insert(id.clone(), user.clone());

    publish(sse, Message::UserCreated(id.clone())).await;

    Ok(with_status(json(&UserResponse::new(&id, &user.name, &user.age)), StatusCode::CREATED))
}

pub async fn update(id: Uuid, user: UserRequest, db: Database, sse: Sse) -> Result<impl warp::Reply, warp::Rejection> {
    let mut repository = db.write().await;

    match repository.exists(&id) {
        false => Ok(with_status(json(&format!("User id: {} not found", &id)), StatusCode::NOT_FOUND)),//issue https://github.com/seanmonstar/warp/issues/77
        true => {
            let user: User = user.into();
            repository.update(id, user.clone());
            publish(sse, Message::UserUpdated(id.clone())).await;
            Ok(with_status(json(&UserResponse::new(&id, &user.name, &user.age)), StatusCode::OK))
        }
    }
}

pub async fn delete(id: Uuid, db: Database, sse: Sse) -> Result<impl warp::Reply, warp::Rejection> {
    let mut repository = db.write().await;

    match repository.exists(&id) {
        false => Ok(StatusCode::NOT_FOUND),
        true => {
            repository.delete(&id);
            publish(sse, Message::UserDeleted(id.clone())).await;
            Ok(StatusCode::OK)
        }
    }
}