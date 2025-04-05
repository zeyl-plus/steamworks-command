use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "steamworks-command")]
#[command(version = "0.1.0")]
#[command(about = "Steamworks Command Tool / Steamworks命令行工具", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// APPID | 应用ID
    #[arg(short, long, default_value_t = 480)]
    pub appid: u32,
}

// 命令
#[derive(Subcommand)]
pub enum Commands {
    /// About User Action | 用户相关操作
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
    /// About App/Game Action | 应用/游戏相关操作
    App {
        #[command(subcommand)]
        action: AppCommands,
    },
    /// About UGC Action | UGC相关操作
    UGC {
        #[command(subcommand)]
        action: UGCCommands,
    },
}

// 用户相关命令
#[derive(Subcommand)]
pub enum UserCommands {
    /// 获取用户信息
    Info,
    /// 获取用户好友列表
    Friends,
}

// 应用相关命令
#[derive(Subcommand)]
pub enum AppCommands {
    /// 获取应用信息
    Info,
}

// UGC相关命令
#[derive(Subcommand)]
pub enum UGCCommands {
    /// 获取单个Item数据
    Item {
        #[arg(short, long)]
        id: u64,
    },
    /// 获取多个Item数据
    Items {
        #[arg(short, long)]
        ids: String,
    },
    /// 分页获取所有Item数据
    All {
        #[arg(short, long)]
        page: u32,
    },
    /// 获取Item下载信息
    DownloadInfo {
        #[arg(short, long)]
        id: u64,
    },
    /// 获取Item状态
    State {
        #[arg(short, long)]
        id: u64,
    },
    /// 订阅Item
    Subscribe {
        #[arg(short, long)]
        id: u64,
    },
    /// 获取订阅的Item ids
    Subscribed,
    /// 取消订阅
    Unsubscribe {
        #[arg(short, long)]
        id: u64,
    },
    /// 下载item
    Download {
        #[arg(short, long)]
        id: u64,
    },
}
