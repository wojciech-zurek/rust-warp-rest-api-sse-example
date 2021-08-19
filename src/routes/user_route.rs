use uuid::Uuid;
use warp::{Filter, path};
use warp::filters::BoxedFilter;

use crate::api::user::UserRequest;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "users" / ..).boxed()
}

pub fn find_all() -> BoxedFilter<()> {
    warp::get().and(path_prefix()).and(warp::path::end()).boxed()
}

pub fn find_by_id() -> BoxedFilter<(Uuid, )> {
    warp::get().and(path_prefix()).and(warp::path::param::<Uuid>()).boxed()
}

pub fn create() -> BoxedFilter<(UserRequest, )> {
    let body = warp::body::content_length_limit(1024).and(warp::body::json());

    warp::post().and(path_prefix()).and(warp::path::end()).and(body).boxed()
}

pub fn update() -> BoxedFilter<(Uuid, UserRequest, )> {
    let body = warp::body::content_length_limit(1024).and(warp::body::json());

    warp::put().and(path_prefix()).and(warp::path::param::<Uuid>()).and(body).boxed()
}

pub fn delete() -> BoxedFilter<(Uuid, )> {
    warp::delete().and(path_prefix()).and(warp::path::param::<Uuid>()).boxed()
}