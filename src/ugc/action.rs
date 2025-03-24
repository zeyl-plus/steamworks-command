use std::time::Duration;

use serde::Serialize;
use steamworks::{Client, PublishedFileId, SingleClient};

use crate::utils::utils::wait_for_response;

// 数据返回类型
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    pub(crate) code: i32,
    pub(crate) result: T,
    pub(crate) message: String,
}

// 订阅Item
pub fn subscribe_item(id: u64, client: Client, single: SingleClient) -> DataResponse<u64> {
    let (tx, rx) = std::sync::mpsc::channel();
    client
        .ugc()
        .subscribe_item(PublishedFileId(id), move |res| {
            tx.send(res.map(|_| id)).unwrap(); // 直接传递原始Result
        });
    wait_for_response(rx, single, 15, Duration::from_millis(100))
        .map(|result| DataResponse {
            code: 0,
            result,
            message: "订阅成功".to_string(),
        })
        .unwrap_or_else(|err| match err {
            err => DataResponse {
                code: -1,
                result: id,
                message: err,
            },
        })
}

// 取消订阅
pub fn unsubscribe_item(id: u64, client: Client, single: SingleClient) -> DataResponse<u64> {
    let (tx, rx) = std::sync::mpsc::channel();
    client
        .ugc()
        .unsubscribe_item(PublishedFileId(id), move |res| {
            tx.send(res.map(|_| id)).unwrap(); // 直接传递原始Result
        });
    wait_for_response(rx, single, 15, Duration::from_millis(100))
        .map(|result| DataResponse {
            code: 0,
            result,
            message: "取消订阅成功".to_string(),
        })
        .unwrap_or_else(|err| match err {
            err => DataResponse {
                code: -1,
                result: id,
                message: err,
            },
        })
}
