use serde::Serialize;
use steamworks::QueryResult;

use super::statistic::StatisticInfo;

#[derive(Debug, Serialize)]
pub struct ItemInfo {
    // 统计信息
    pub views: u64,
    pub subscriptions: u64,
    pub favorited: u64,
    pub preview_url: String,
    // QueryResult属性
    pub id: u64,
    pub publishedfileid: u64,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub file_size: u32,
    pub score: f32,
    pub children: Option<Vec<u64>>,
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
            favorited: stat.favorited,
            preview_url: stat.preview_url,
            id: result.published_file_id.0,
            publishedfileid: result.published_file_id.0,
            title: result.title,
            description: result.description,
            score: result.score,
            tags,
            children: None,
            file_size: result.file_size,
            time_created: result.time_created,
            time_updated: result.time_updated,
        }
    }
    // 设置子项
    pub(crate) fn set_children(mut self, children: Vec<u64>) -> Self {
        self.children = Some(children.clone());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    // 当前页
    pub page: u32,
    // 每页数量
    pub size: u32,
    // 总数
    pub total: u32,
    // 数据
    pub items: T,
}
