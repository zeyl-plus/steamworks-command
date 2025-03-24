use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // APP ID
    #[arg(short, long)]
    pub(crate) app: u32,

    // Item IDs
    #[arg(short, long)]
    pub(crate) item: String,

    // 操作类型
    #[arg(long, default_value = "get_item")]
    pub(crate) action: String,

    // 分页
    #[arg(short, long, default_value_t = 0)]
    pub(crate) page: u32,
}

pub struct SteamArgs {
    pub(crate) app_id: u32,
    pub(crate) item_ids: Vec<u64>,
    pub(crate) action: ActionType,
    pub(crate) page: u32,
}

// 操作类型
pub enum ActionType {
    GetItem,
    GetItems,
    GetAll,
    ItemInstallInfo,
    ItemDownloadInfo,
    AppInstallInfo,
    GetItemState,
    SubscribeItem,
    SubscribedItems,
    UnsubscribeItem,
    UnsubscribeItems,
}

impl ActionType {
    pub fn from_str(s: &str) -> ActionType {
        match s {
            "get_item" => ActionType::GetItem,
            "get_items" => ActionType::GetItems,
            "get_all" => ActionType::GetAll,
            "item_install_info" => ActionType::ItemInstallInfo,
            "item_download_info" => ActionType::ItemDownloadInfo,
            "app_install_info" => ActionType::AppInstallInfo,
            "get_item_state" => ActionType::GetItemState,
            "subscribe_item" => ActionType::SubscribeItem,
            "subscribed_items" => ActionType::SubscribedItems,
            "unsubscribe_item" => ActionType::UnsubscribeItem,
            "unsubscribe_items" => ActionType::UnsubscribeItems,
            _ => ActionType::GetItem,
        }
    }
}
