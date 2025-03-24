use serde::Serialize;
use steamworks::QueryResult;

use super::statistic::StatisticInfo;

#[derive(Debug, Serialize)]
pub struct ItemInfo {
    // 统计信息
    pub views: u64,
    pub subscriptions: u64,
    pub favorites: u64,
    pub preview_url: String,
    // QueryResult属性
    pub published_file_id: u64,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub file_size: u32,
    pub score: f32,
    pub time_created: u32,
    pub time_updated: u32,
}

impl ItemInfo {
    pub(crate) fn new(stat: StatisticInfo, result: QueryResult) -> Self {
        let tags: String = result
            .tags
            .iter()
            .map(|tag| tag.to_string())
            .collect::<Vec<_>>()
            .join(",");

        ItemInfo {
            views: stat.views,
            subscriptions: stat.subscriptions,
            favorites: stat.favorites,
            preview_url: stat.preview_url,
            published_file_id: result.published_file_id.0,
            title: result.title,
            // description: "".to_string(),
            description: result.description,
            score: result.score,
            tags,
            file_size: result.file_size,
            time_created: result.time_created,
            time_updated: result.time_updated,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    pub page: u32,
    pub total: u32,
    pub items: T,
}

// impl ItemList {
//     pub(crate) fn new(list: ItemList) -> Self {
//         list
//     }
// }
