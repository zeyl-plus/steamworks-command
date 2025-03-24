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
    query::{get_all, get_item, get_items},
    state::{get_item_state, item_download_info},
};
use utils::args::get_args;

fn main() {
    // 获取参数
    let args = get_args(Args::parse());

    println!("APP_ID: {}", args.app_id);
    println!("ITEM_IDs: {:?}", args.item_ids);

    // TEST ID
    // 1843445119,2743616455
    let (client, single) = Client::init_app(args.app_id).expect("Failed to initialize steamworks");
    match args.action {
        // 获取单个item
        ActionType::GetItem => {
            println!("--------------get_item--------------");
            let res = get_item(args.item_ids[0], client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &res.ok());
        }
        // 获取多个item
        ActionType::GetItems => {
            println!("--------------get_items--------------");
            let res = get_items(args.item_ids, client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &res.ok());
        }
        // 获取所有item
        ActionType::GetAll => {
            println!("--------------get_all--------------");
            let res = get_all(args.page, args.app_id, client.clone(), single);
            let _ = serde_json::to_writer(io::stdout(), &res.ok());
        }
        ActionType::ItemDownloadInfo => {
            println!("--------------item_download_info--------------");
            let res = item_download_info(args.item_ids[0], client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        // 获取item状态
        ActionType::GetItemState => {
            println!("--------------get_item_state--------------");
            let res = get_item_state(args.item_ids[0], client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res.ok());
        }
        // 判断游戏是否已安装
        ActionType::AppInstallInfo => {
            println!("--------------app_install_info--------------");
            let res = app_install_info(args.app_id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        _ => {
            eprintln!("Invalid action")
        }
    }
}
