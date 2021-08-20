use std::env;

use warp::Filter;

use emitter::sse_emitter::create_sse;
use emitter::sse_emitter::with_sse;
use crate::repository::user_repository::create_db;
use crate::repository::user_repository::with_db;

pub mod routes;
mod models;
pub mod repository;
mod api;
pub mod handlers;
pub mod emitter;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");

    pretty_env_logger::init();

    let db = create_db();
    let sse = create_sse();

    let log = warp::log("any");

    let api = find_all!(db.clone())
        .or(find_by_id!(db.clone()))
        .or(create!(db.clone(), sse.clone()))
        .or(update!(db.clone(), sse.clone()))
        .or(delete!(db, sse.clone()))
        .or(sse!(sse));

    let api = api.with(log);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}