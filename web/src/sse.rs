use axum::{
    response::sse::{Event, Sse},
    Router,
};
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use std::{convert::Infallible, path::PathBuf, time::Duration};
use tokio_stream::StreamExt as _;
use tower_http::{trace::TraceLayer};
use tracing_subscriber::{util::SubscriberInitExt};
pub async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
#[derive(Deserialize, Debug)]
struct Info {
    name: String,
    age: u8,
}
async fn json_handler2(Json(info): Json<HashMap<String, String>>) -> String {
    format!("info: {info:?}")
}
async fn json_handler(Json(info): Json<Info>) -> String {
    format!("info: {info:?}")
}