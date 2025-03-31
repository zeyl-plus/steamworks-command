mod app;
mod commands;
mod models;
mod ugc;
mod utils;
use clap::Parser;
use commands::app::handle_app_commands;
use commands::ugc::handle_ugc_commands;
use commands::user::handle_user_commands;
use models::command::Cli;
use models::command::Commands;
use std::io;
use steamworks::Client;
use utils::response::DataResponse;

// 初始化steam client
fn init_app(app_id: u32) -> Client {
    let client = Client::init_app(app_id)
        .map_err(|e| {
            let res = DataResponse::new(-1, "Steamworks initialization failed", e.to_string());
            let _ = serde_json::to_writer(io::stdout(), &res);
            std::process::exit(0);
        })
        .unwrap();
    return client;
}

fn main() {
    // TEST ID
    // 1791288541,1843445119,2743616455
    let cli = Cli::parse();

    let client = init_app(cli.appid);

    // 检测appid
    if cli.appid == 0 {
        let res = DataResponse::error("Invalid app_id", "无效的app_id");
        let _ = serde_json::to_writer(io::stdout(), &res);
        std::process::exit(0);
    }

    match &cli.command {
        Commands::UGC { action } => handle_ugc_commands(action, client),
        Commands::App { action } => handle_app_commands(action, client),
        Commands::User { action } => handle_user_commands(action, client),
    }
}
