use std::time::Duration;

use steamworks::{AppIDs, AppId, Client, PublishedFileId, UGCQueryType, UGCType};

use crate::{
    models::{
        item::{ItemInfo, Pagination},
        statistic::StatisticInfo,
    },
    utils::utils::wait_for_response,
};

// 获取单个Item数据
pub fn get_item(id: u64, client: Client) -> Result<ItemInfo, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    let query = client
        .ugc()
        .query_item(PublishedFileId(id))
        .expect("Failed to query item")
        .include_long_desc(true);
    query.fetch(move |res| {
        let _ = tx.send(match res {
            Ok(item) => {
                let statistics: StatisticInfo = StatisticInfo::new(0, &item);
                let item = item.get(0).expect("没有找到数据");
                let data: ItemInfo = ItemInfo::new(statistics, item);
                // println!("数据信息: {:?}", data);
                Ok(data)
            }
            Err(err) => {
                println!("Steam Error: {:?}", err);
                Err(err)
            }
        });
    });
    wait_for_response(rx, client, 15, Duration::from_millis(100))
}

// 获取多个Item数据
pub fn get_items(ids: Vec<u64>, client: Client) -> Result<Vec<ItemInfo>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let query = client
        .ugc()
        .query_items(
            ids.iter()
                .map(|id| PublishedFileId(*id))
                .collect::<Vec<_>>(),
        )
        .expect("Failed to query items")
        .include_long_desc(true);

    query.fetch(move |res| {
        let mut items_list = Vec::new();
        let send_result = match res {
            Ok(items) => {
                for (index, query) in items.iter().enumerate() {
                    if let Some(item) = query {
                        let statistics = StatisticInfo::new(index, &items);
                        items_list.push(ItemInfo::new(statistics, item));
                    }
                }
                Ok(items_list)
            }
            Err(err) => {
                println!("Steam Error: {:?}", err);
                Err(err)
            }
        };
        let _ = tx.send(send_result);
    });

    wait_for_response(rx, client, 15, Duration::from_millis(100))
}

// 分页获取所有Item数据
pub fn get_all(
    page: u32,
    app_id: u32,
    client: Client,
) -> Result<Pagination<Vec<ItemInfo>>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let query = client
        .ugc()
        .query_all(
            UGCQueryType::RankedByVote,
            UGCType::ItemsReadyToUse,
            AppIDs::CreatorAppId(AppId(app_id)),
            page,
        )
        .expect("Failed to query all")
        .include_long_desc(true);

    query.fetch(move |res| {
        let mut items_list = Vec::new();
        let send_result = match res {
            Ok(items) => {
                for (index, query) in items.iter().enumerate() {
                    // println!(
                    //     "数据信息({:?}): {:?}",
                    //     index,
                    //     items.get(index.try_into().unwrap()).unwrap().title
                    // );
                    if let Some(item_query) = query {
                        let statistics = StatisticInfo::new(index, &items);
                        items_list.push(ItemInfo::new(statistics, item_query));
                    }
                }
                Ok(Pagination {
                    page,
                    size: items.returned_results(),
                    total: items.total_results(),
                    items: items_list,
                })
            }
            Err(err) => {
                println!("Steam Error: {:?}", err);
                Err(err)
            }
        };
        let _ = tx.send(send_result);
    });

    wait_for_response(rx, client, 15, Duration::from_millis(100))
}
