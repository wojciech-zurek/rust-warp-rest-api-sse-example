use warp::Filter;
use warp::filters::BoxedFilter;
use warp::path;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "sse" / ..).boxed()
}

pub fn sse() -> BoxedFilter<()> {
    warp::get().and(path_prefix()).and(warp::path::end()).boxed()
}