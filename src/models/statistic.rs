use steamworks::{QueryResults, UGCStatisticType};

#[derive(Debug)]
pub struct StatisticInfo {
    pub views: u64,
    pub subscriptions: u64,
    pub favorited: u64,
    pub preview_url: String,
}

impl StatisticInfo {
    pub(crate) fn new(index: usize, items: &QueryResults) -> Self {
        let index = index.try_into().unwrap();
        let preview_url = items.preview_url(index).unwrap();
        let favorited = items.statistic(index, UGCStatisticType::Favorites).unwrap();
        let subscriptions = items
            .statistic(index, UGCStatisticType::Subscriptions)
            .unwrap();
        let views = items
            .statistic(index, UGCStatisticType::UniqueWebsiteViews)
            .unwrap();
        StatisticInfo {
            views,
            subscriptions,
            favorited,
            preview_url: String::from(preview_url),
        }
    }
}
