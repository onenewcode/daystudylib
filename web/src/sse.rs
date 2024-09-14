use axum::{
    extract::Path, http::StatusCode, response::{sse::{Event, Sse}, IntoResponse}, Json
};
use serde::{Deserialize, Serialize};
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use headers::UserAgent;
use std::{collections::HashMap, convert::Infallible,time::Duration};
use tokio_stream::StreamExt as _;
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
// 需要添加序列化，反序列化
#[derive(Serialize,Deserialize, Debug)]
pub  struct Info {
    name: String,
    age: u8,
}
// 路径参数
// /path2/:name/:age
pub  async fn path_handler2(Path((name, age)): Path<(String, i64)>) -> String {
    format!("name: {name}, age: {age}")
}
// 获取请求头
pub  async fn header_handler(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    format!("header.user_agent: {user_agent:?}")
}
// 请求体
pub async fn json_handler2(Json(info): Json<HashMap<String, String>>) -> String {
    format!("info: {info:?}")
}
pub async fn json_handler(Json(info): Json<Info>) -> String {
    format!("info: {info:?}")
}
pub async fn create_info(
    Json(info): Json<Info>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(info))
}