use std::env;

use warp::Filter;

use crate::repository::user_repository::create_db;
use crate::repository::user_repository::with_db;

pub mod routes;
mod models;
pub mod repository;
mod api;
pub mod handlers;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");

    pretty_env_logger::init();

    let db = create_db();

    let log = warp::log("any");

    let api = find_all!(db.clone())
        .or(find_by_id!(db.clone()))
        .or(create!(db.clone()))
        .or(update!(db.clone()))
        .or(delete!(db));

    let routes = api.with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}