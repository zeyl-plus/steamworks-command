use std::io;

use steamworks::Client;

use crate::{
    models::command::UGCCommands,
    ugc::{
        action::{download_item, subscribe_item, subscribed_items, unsubscribe_item},
        query::{get_all, get_item, get_items},
        state::{get_item_state, item_download_info},
    },
    utils::response::DataResponse,
};

// 处理UGC命令
pub fn handle_ugc_commands(action: &UGCCommands, client: Client) {
    match action {
        UGCCommands::Item { id } => {
            let res = get_item(*id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        UGCCommands::Items { ids } => {
            let ids: Vec<u64> = ids.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
            let res = get_items(ids.clone(), client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        UGCCommands::All { page } => {
            let app = client.utils().app_id().0;
            let res = get_all(*page, app, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        UGCCommands::DownloadInfo { id } => {
            let res = item_download_info(*id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
        }
        UGCCommands::State { id } => {
            let res = get_item_state(*id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res.ok()));
        }
        UGCCommands::Subscribe { id } => {
            let res = subscribe_item(*id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        UGCCommands::Subscribed => {
            let res = subscribed_items(client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        UGCCommands::Unsubscribe { id } => {
            let res = unsubscribe_item(*id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        UGCCommands::Download { id } => {
            let res = download_item(*id, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
    }
}
