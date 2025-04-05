use serde::Serialize;
use steamworks::{AppId, Client};

use crate::utils::response::DataResponse;

#[derive(Debug, Serialize)]
pub struct AppInstallInfo {
    pub(crate) id: u32,
    pub(crate) install_dir: String,
    pub(crate) installed: bool,
    pub(crate) language: String,
    pub(crate) owner: u64,
    pub(crate) build_id: i32,
}

// 获取游戏安装信息
pub fn app_install_info(app_id: u32, client: Client) -> DataResponse<AppInstallInfo> {
    let id = AppId(app_id);
    let app = client.apps();
    let install_dir = app.app_install_dir(id);
    let installed = app.is_app_installed(id);
    let language = app.current_game_language();
    let owner = app.app_owner();
    let build_id = app.app_build_id();
    DataResponse::success(AppInstallInfo {
        id: app_id,
        install_dir,
        language,
        owner: owner.raw(),
        installed,
        build_id,
    })
}
