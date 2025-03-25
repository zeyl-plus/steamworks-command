mod app;
mod commands;
mod models;
mod ugc;
mod utils;
use clap::Parser;
use commands::app::handle_app_commands;
use commands::ugc::handle_ugc_commands;
use commands::user::handle_user_commands;
use models::command::Cli;
use models::command::Commands;
use std::io;
use steamworks::{Client, SingleClient};
use utils::response::DataResponse;

// 初始化steam client
fn init_app(app_id: u32) -> (Client, SingleClient) {
    let (client, single) = Client::init_app(app_id)
        .map_err(|e| {
            let res = DataResponse::new(-1, "Steamworks initialization failed", e.to_string());
            let _ = serde_json::to_writer(io::stdout(), &res);
            std::process::exit(0);
        })
        .unwrap();
    return (client, single);
}

fn main() {
    // TEST ID
    // 1791288541,1843445119,2743616455
    let cli = Cli::parse();

    let (client, single) = init_app(cli.appid);

    // 检测appid
    if cli.appid == 0 {
        let res = DataResponse::error("Invalid app_id", "无效的app_id");
        let _ = serde_json::to_writer(io::stdout(), &res);
        std::process::exit(0);
    }

    match &cli.command {
        Commands::UGC { action } => handle_ugc_commands(action, client, single),
        Commands::App { action } => handle_app_commands(action, client),
        Commands::User { action } => handle_user_commands(action, client),
    }
    // println!("APP_ID: {}", args.app_id);
    // println!("ITEM_IDs: {:?}", args.item_ids);
    // println!("ACTION: {:?}", args.action);

    // match args.action {
    //     // 获取单个item
    //     ActionType::GetItem => {
    //         // println!("--------------get_item--------------");
    //         let res = get_item(args.item_ids[0], client.clone(), single);
    //         let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
    //     }
    //     // 获取多个item
    //     ActionType::GetItems => {
    //         // println!("--------------get_items--------------");
    //         let res = get_items(args.item_ids, client.clone(), single);
    //         let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
    //     }
    //     // 获取所有item
    //     ActionType::GetAll => {
    //         // println!("--------------get_all--------------");
    //         let res = get_all(args.page, args.app_id, client.clone(), single);
    //         let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
    //     }
    //     // 获取item下载信息
    //     ActionType::ItemDownloadInfo => {
    //         // println!("--------------item_download_info--------------");
    //         let res = item_download_info(args.item_ids[0], client.clone());
    //         let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
    //     }
    //     // 获取item状态
    //     ActionType::GetItemState => {
    //         // println!("--------------get_item_state--------------");
    //         let res = get_item_state(args.item_ids[0], client.clone());
    //         let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
    //     }
    //     // 订阅item
    //     ActionType::SubscribeItem => {
    //         // println!("--------------subscribe_item--------------");
    //         let res = subscribe_item(args.item_ids[0], client.clone(), single);
    //         let _ = serde_json::to_writer(io::stdout(), &res);
    //     }
    //     // 取消订阅item
    //     ActionType::UnsubscribeItem => {
    //         // println!("--------------unsubscribe_item--------------");
    //         let res = unsubscribe_item(args.item_ids[0], client.clone(), single);
    //         let _ = serde_json::to_writer(io::stdout(), &res);
    //     }
    //     // 获取已订阅的item
    //     ActionType::SubscribedItems => {
    //         // println!("--------------subscribed_items--------------");
    //         let res = subscribed_items(client);
    //         let _ = serde_json::to_writer(io::stdout(), &res);
    //     }
    //     // 获取游戏安装信息
    //     ActionType::AppInstallInfo => {
    //         // println!("--------------app_install_info--------------");
    //         let res = app_install_info(args.app_id, client.clone());
    //         let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
    //     }
    //     // 下载item
    //     ActionType::DownloadItem => {
    //         // println!("--------------download_item--------------");
    //         let res = download_item(args.item_ids[0], client.clone());
    //         let _ = serde_json::to_writer(io::stdout(), &res);
    //     }
    //     _ => {
    //         eprintln!("Invalid action");
    //         let res = DataResponse {
    //             code: -1,
    //             result: "Invalid action".to_string(),
    //             message: "非有效的操作类型".to_string(),
    //         };
    //         let _ = serde_json::to_writer(io::stdout(), &res);
    //     }
    // }
}
