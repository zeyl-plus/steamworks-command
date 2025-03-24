use steamworks::{QueryResults, UGCStatisticType};

#[derive(Debug)]
pub struct StatisticInfo {
    pub views: u64,
    pub subscriptions: u64,
    pub favorites: u64,
    pub preview_url: String,
}

impl StatisticInfo {
    pub(crate) fn new(items: &QueryResults) -> Self {
        let preview_url = items.preview_url(0).unwrap();
        let favorites = items.statistic(0, UGCStatisticType::Favorites).unwrap();
        let subscriptions = items.statistic(0, UGCStatisticType::Subscriptions).unwrap();
        let views = items
            .statistic(0, UGCStatisticType::UniqueWebsiteViews)
            .unwrap();
        StatisticInfo {
            views: views,
            subscriptions: subscriptions,
            favorites: favorites,
            preview_url: String::from(preview_url),
        }
    }
}
