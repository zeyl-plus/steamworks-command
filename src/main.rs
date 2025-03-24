mod app;
mod models;
mod ugc;
mod utils;
use app::install::app_install_info;
use clap::Parser;
use models::args::{ActionType, Args};
use std::io;
use steamworks::Client;
use ugc::{
    action::{download_item, subscribe_item, subscribed_items, unsubscribe_item},
    query::{get_all, get_item, get_items},
    state::{get_item_state, item_download_info},
};
use utils::{args::get_args, response::DataResponse};

fn main() {
    // 获取参数
    let args = get_args(Args::parse());

    // println!("APP_ID: {}", args.app_id);
    // println!("ITEM_IDs: {:?}", args.item_ids);
    // println!("ACTION: {:?}", args.action);

    // TEST ID
    // 1791288541,1843445119,2743616455
    let (client, single) = Client::init_app(args.app_id)
        .map_err(|e| {
            let res = DataResponse {
                code: -1,
                result: "Steamworks initialization failed".to_string(),
                message: e.to_string(),
            };
            let _ = serde_json::to_writer(io::stdout(), &res);
            std::process::exit(0);
        })
        .unwrap();

    match args.action {
        // 获取单个item
        ActionType::GetItem => {
            // println!("--------------get_item--------------");
            let res = get_item(args.item_ids[0], client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        // 获取多个item
        ActionType::GetItems => {
            // println!("--------------get_items--------------");
            let res = get_items(args.item_ids, client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        // 获取所有item
        ActionType::GetAll => {
            // println!("--------------get_all--------------");
            let res = get_all(args.page, args.app_id, client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        // 获取item下载信息
        ActionType::ItemDownloadInfo => {
            // println!("--------------item_download_info--------------");
            let res = item_download_info(args.item_ids[0], client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
        }
        // 获取item状态
        ActionType::GetItemState => {
            // println!("--------------get_item_state--------------");
            let res = get_item_state(args.item_ids[0], client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        // 订阅item
        ActionType::SubscribeItem => {
            // println!("--------------subscribe_item--------------");
            let res = subscribe_item(args.item_ids[0], client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        // 取消订阅item
        ActionType::UnsubscribeItem => {
            // println!("--------------unsubscribe_item--------------");
            let res = unsubscribe_item(args.item_ids[0], client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        // 获取已订阅的item
        ActionType::SubscribedItems => {
            // println!("--------------subscribed_items--------------");
            let res = subscribed_items(client);
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        // 获取游戏安装信息
        ActionType::AppInstallInfo => {
            // println!("--------------app_install_info--------------");
            let res = app_install_info(args.app_id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
        }
        // 下载item
        ActionType::DownloadItem => {
            // println!("--------------download_item--------------");
            let res = download_item(args.item_ids[0], client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        _ => {
            eprintln!("Invalid action");
            let res = DataResponse {
                code: -1,
                result: "Invalid action".to_string(),
                message: "非有效的操作类型".to_string(),
            };
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
    }
}
