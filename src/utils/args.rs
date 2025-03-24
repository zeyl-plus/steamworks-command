use crate::models::args::{ActionType, Args, SteamArgs};

// 参数处理
pub fn get_args(args: Args) -> SteamArgs {
    // app id
    let app_id = if args.app != 0 { args.app } else { 779340 };
    // item id数组
    let item_ids: Vec<u64> = args
        .item
        .split(",")
        .map(|i| i.parse::<u64>().unwrap())
        .collect();
    // 操作类型
    let action = ActionType::from_str(&args.action);
    // 分页
    let page = if args.page != 0 { args.page } else { 1 };

    SteamArgs {
        app_id,
        item_ids,
        action,
        page,
    }
}
