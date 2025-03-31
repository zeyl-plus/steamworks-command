// 构建脚本
use std::fs;

fn main() {
    // SDK DLL 的路径
    let sdk_dll_path =
        "./steamworks-rs/steamworks-sys/lib/steam/redistributable_bin/win64/steam_api64.dll";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir).parent().unwrap();

    // 复制 DLL 到 target/release 或 target/debug
    fs::copy(sdk_dll_path, target_dir.join("steam_api64.dll")).unwrap();
}
