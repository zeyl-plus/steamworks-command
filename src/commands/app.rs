use std::io;

use steamworks::Client;

use crate::{
    app::app::{app_install_info, app_start},
    models::command::AppCommands,
};

// 处理APP应用相关命令
pub fn handle_app_commands(action: &AppCommands, client: Client) {
    match action {
        AppCommands::Info => {
            let app = client.utils().app_id().0;
            let res = app_install_info(app, client.clone());
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
        AppCommands::Start => {
            let res = app_start(client);
            let _ = serde_json::to_writer(io::stdout(), &res);
        }
    }
}
