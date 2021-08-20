use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use warp::Filter;

use crate::api::sse::Message;

pub type Sse = Arc<Mutex<SseEmitter>>;

pub struct SseEmitter {
    inner: Vec<mpsc::UnboundedSender<Message>>,
}

impl SseEmitter {
    pub fn new() -> Self {
        SseEmitter {
            inner: Vec::new()
        }
    }

    pub fn insert(&mut self, tx: mpsc::UnboundedSender<Message>) {
        self.inner.push(tx);
    }
}

impl Deref for SseEmitter {
    type Target = Vec<mpsc::UnboundedSender<Message>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SseEmitter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub fn create_sse() -> Sse {
    Arc::new(Mutex::new(SseEmitter::new()))
}

pub fn with_sse(sse: Sse) -> impl Filter<Extract=(Sse, ), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || sse.clone())
}

pub async fn publish(sse: Sse, message: Message) {
    tokio::spawn(async move {
        sse.lock().await.retain(|tx| {
            tx.send(message.clone()).is_ok()
        });
    });
}