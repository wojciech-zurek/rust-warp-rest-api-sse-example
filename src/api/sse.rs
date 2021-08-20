use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub enum Message {
    Hello,
    UserCreated(Uuid),
    UserUpdated(Uuid),
    UserDeleted(Uuid),
}

#[macro_export]
macro_rules! sse {
($sse:expr) => {
    self::routes::sse_route::sse().and(with_sse($sse)).and_then(self::handlers::sse_handler::connect)
}
}