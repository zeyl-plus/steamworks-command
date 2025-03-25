use std::{thread, time::Duration};

use base64::{Engine as _, engine::general_purpose};

use steamworks::SingleClient;

pub fn clear() {
    unsafe {
        std::env::remove_var("SteamAppId");
        std::env::remove_var("SteamGameId");
    };
}

// 等待操作响应
pub fn wait_for_response<T>(
    rx: std::sync::mpsc::Receiver<Result<T, steamworks::SteamError>>,
    single: SingleClient,
    max_retries: usize,
    sleep_duration: Duration,
) -> Result<T, String> {
    for _ in 0..max_retries {
        single.run_callbacks();
        match rx.try_recv() {
            Ok(Ok(res)) => {
                drop(rx);
                drop(single);
                crate::utils::utils::clear();
                return Ok(res);
            }
            Ok(Err(_)) => eprintln!("未获取到数据"),
            Err(_) => {
                // 超时日志
            }
        }
        thread::sleep(sleep_duration);
    }
    Err("获取数据超时".to_string())
}

// 将字节数组转换为base64字符串
pub fn to_base64(data: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(&data)
}
