use axum::{
    extract::Path,
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    Json,
};
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt as _};
pub async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());
    // 构建自己的流，用管道传输

    // 创建一个无界管道,不能使用tokio的管道，因为它是异步的

    let (tx, mut rx) = mpsc::unbounded_channel();

    // 创建一个独立任务任务来发送消息
    // 同步代码块，不能运行异步线程
    tokio::spawn(async move {
        let mut i = 0;
        loop {
            tx.send(format!("Message {}", i)).unwrap();
            i += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    // 生成一个流，用于向客户端传输
    let stream = UnboundedReceiverStream::new(rx)
        .map(|data| Event::default().data(data))
        .map(Ok)
        .throttle(Duration::from_secs(1));
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
// 需要添加序列化，反序列化
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    name: String,
    age: u8,
}
// 路径参数
// /path2/:name/:age
pub async fn path_handler2(Path((name, age)): Path<(String, i64)>) -> String {
    format!("name: {name}, age: {age}")
}
// 获取请求头

pub async fn header_handler(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    format!("header.user_agent: {user_agent:?}")
}
// 请求体

pub async fn json_handler2(Json(info): Json<HashMap<String, String>>) -> String {
    format!("info: {info:?}")
}
pub async fn json_handler(Json(info): Json<Info>) -> String {
    format!("info: {info:?}")
}
pub async fn create_info(Json(info): Json<Info>) -> impl IntoResponse {
    (StatusCode::OK, Json(info))
}
