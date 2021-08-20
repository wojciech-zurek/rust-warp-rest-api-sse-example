use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::sse::Event;

use crate::api::sse::Message;
use crate::emitter::sse_emitter::Sse;

pub async fn connect(sse: Sse) -> Result<impl warp::Reply, warp::Rejection> {
    let stream = stream(sse).await;
    Ok(warp::sse::reply(warp::sse::keep_alive().stream(stream)))
}

async fn stream(sse: Sse) -> impl Stream<Item=Result<Event, warp::Error>> + Send + 'static {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tx.send(Message::Hello).unwrap();

    let mut sse = sse.lock().await;
    sse.insert(tx);

    rx.map(|msg| {
        match msg {
            Message::Hello => Ok(Event::default().event("system").json_data(format!("{:?}", msg)).unwrap()),
            _ => Ok(Event::default().event("user").json_data(format!("{:?}", msg)).unwrap()),
        }
    })
}