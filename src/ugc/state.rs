use serde::Serialize;
use steamworks::{Client, ItemState, PublishedFileId};

// item 状态
#[derive(Debug, Serialize)]
pub struct ItemStatus {
    pub(crate) id: u64,
    pub(crate) folder: String,
    pub(crate) size: u64,
    pub(crate) time_updated: u32,
    pub(crate) subscribed: bool,
    pub(crate) installed: bool,
    pub(crate) downloading: bool,
    pub(crate) states: Vec<String>,
}

// 下载进度
#[derive(Debug, Serialize)]
pub struct DownloadInfo {
    pub(crate) total: u64,
    pub(crate) current: u64,
}

// 获取下载信息
pub fn item_download_info(id: u64, client: Client) -> DownloadInfo {
    let (total, current) = client
        .ugc()
        .item_download_info(PublishedFileId(id))
        .map_or_else(|| (0, 0), |(total, current)| (total, current));
    DownloadInfo { total, current }
}

// 获取Item状态
pub fn get_item_state(id: u64, client: Client) -> Result<ItemStatus, String> {
    let pub_id = PublishedFileId(id);
    let item_state = client.ugc().item_state(pub_id);
    let install_info = client.ugc().item_install_info(pub_id);

    let (folder, size_on_disk, timestamp) = install_info.map_or_else(
        || {
            eprintln!("No install info found for item {:?}", id);
            ("".to_string(), 0, 0)
        },
        |info| (info.folder, info.size_on_disk, info.timestamp),
    );
    let mut states = Vec::new();
    if item_state.contains(ItemState::SUBSCRIBED) {
        states.push("SUBSCRIBED".into());
    }
    if item_state.contains(ItemState::LEGACY_ITEM) {
        states.push("LEGACY_ITEM".into());
    }
    if item_state.contains(ItemState::INSTALLED) {
        states.push("INSTALLED".into());
    }
    if item_state.contains(ItemState::NEEDS_UPDATE) {
        states.push("NEEDS_UPDATE".into());
    }
    if item_state.contains(ItemState::DOWNLOADING) {
        states.push("DOWNLOADING".into());
    }
    if item_state.contains(ItemState::DOWNLOAD_PENDING) {
        states.push("DOWNLOAD_PENDING".into());
    }
    if states.len() == 0 {
        states.push("NONE".into());
    }
    Ok(ItemStatus {
        id,
        states,
        folder,
        size: size_on_disk,
        time_updated: timestamp,
        downloading: item_state.contains(ItemState::DOWNLOADING),
        installed: item_state.contains(ItemState::INSTALLED),
        subscribed: item_state.contains(ItemState::SUBSCRIBED),
    })
}
