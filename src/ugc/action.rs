use std::time::Duration;

use steamworks::{Client, PublishedFileId, SingleClient};

use crate::utils::{response::DataResponse, utils::wait_for_response};
// 订阅Item
pub fn subscribe_item(id: u64, client: Client, single: SingleClient) -> DataResponse<u64> {
    let (tx, rx) = std::sync::mpsc::channel();
    client
        .ugc()
        .subscribe_item(PublishedFileId(id), move |res| {
            tx.send(res.map(|_| id)).unwrap(); // 直接传递原始Result
        });
    wait_for_response(rx, single, 15, Duration::from_millis(100))
        .map(|result| DataResponse::new(0, result, "订阅成功"))
        .unwrap_or_else(|err| match err {
            err => DataResponse::error(id, err),
        })
}
// 获取订阅的Item ids
pub fn subscribed_items(client: Client) -> DataResponse<Vec<u64>> {
    let ids = client
        .ugc()
        .subscribed_items()
        .iter()
        .map(|id| id.0)
        .collect();
    DataResponse::success(ids)
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
        .map(|result| DataResponse::new(0, result, "取消订阅成功"))
        .unwrap_or_else(|err| match err {
            err => DataResponse::error(id, err),
        })
}

// 下载item
pub fn download_item(id: u64, client: Client) -> DataResponse<bool> {
    let download = client.ugc().download_item(PublishedFileId(id), true);
    if download {
        DataResponse::new(0, download, "下载成功")
    } else {
        DataResponse::error(download, "下载失败")
    }
}
