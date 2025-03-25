use std::{fmt::Debug, io};

use serde::Serialize;
use steamworks::{Client, FriendFlags};

use crate::{
    models::command::UserCommands,
    utils::{response::DataResponse, utils::to_base64},
};

#[derive(Debug, Serialize)]
struct UserInfo {
    id: u64,
    level: u32,
    logged_on: bool,
    name: String,
    avatar: String,
}

#[derive(Debug, Serialize)]
struct FriendInfo {
    id: u64,
    name: String,
    avatar: String,
}

pub fn handle_user_commands(action: &UserCommands, client: Client) {
    match action {
        UserCommands::Info => {
            let level = client.user().level();
            let logged_on = client.user().logged_on();
            let steamid = client.user().steam_id();
            let name = client.friends().name();
            let friend = client.friends().get_friend(steamid);
            let avatar = friend.medium_avatar().unwrap_or(vec![]);
            let avatar_base64 = to_base64(avatar);
            let res = UserInfo {
                id: steamid.raw(),
                name,
                level,
                logged_on,
                avatar: avatar_base64,
            };
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
        }
        UserCommands::Friends => {
            let friends = client.friends().get_friends(FriendFlags::IMMEDIATE);
            let res = friends
                .iter()
                .map(|friend| {
                    let avatar = friend.medium_avatar().unwrap_or(vec![]);
                    let avatar_base64 = to_base64(avatar);
                    FriendInfo {
                        id: friend.id().raw(),
                        name: friend.name(),
                        avatar: avatar_base64,
                    }
                })
                .collect::<Vec<FriendInfo>>();
            let _ = serde_json::to_writer(io::stdout(), &DataResponse::success(res));
        }
    }
}
