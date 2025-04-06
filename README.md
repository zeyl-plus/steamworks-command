# Steamworks Command Line Tool

[![Version](https://img.shields.io/badge/Version-0.1.0-green.svg)](https://gitcode.com/zeyl/steamworks-command)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

一个基于 Steamworks RS 的命令行工具，提供 Steamworks API 的便捷访问接口，主要用于管理用户生成内容(如：MOD、MOD 合集等)。

## 安装指南

### 从源码构建

```bash
# 克隆仓库
git clone https://gitcode.com/zeyl/steamworks-command.git
cd steamworks-command
# 本地调试
cargo run -- --appid 779340 ugc item --id 1843445119
# 编译发布版本
cargo build --release
```

### 下载

从 [Release 页面](https://gitcode.com/zeyl/steamworks-command/releases) 下载二进制文件

## 快速开始

```bash
# 查询UGC内容
./steamworks-command.exe --appid 779340 ugc item --id 1843445119
```

## 使用文档

### 命令选项

```bash
Steamworks Command Tool

Usage: steamworks-command.exe [OPTIONS] <COMMAND>

Commands:
  user  About User Action | 用户相关操作
  app   About App/Game Action | 应用/游戏相关操作
  ugc   About UGC Action | UGC相关操作
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --appid <APPID>  APPID | 应用ID [default: 480]
  -h, --help           Print help
  -V, --version        Print version
```

### UGC 操作

```bash
# 获取单个UGC项目详情
./steamworks-command.exe ugc item --id <UGC_ID>
# 批量获取UGC项目
./steamworks-command.exe ugc items --id <ID1>,<ID2>,<ID3>
# 分页查询所有UGC项目
./steamworks-command.exe --appid <APP_ID> ugc all --page 1
# 获取UGC项目状态
./steamworks-command.exe ugc state --id <UGC_ID>
# 下载UGC内容
./steamworks-command.exe ugc download --id <UGC_ID>
# 下载UGC信息
./steamworks-command.exe ugc download-info --id <UGC_ID>
# 管理内容订阅
./steamworks-command.exe ugc subscribe --id <UGC_ID>
./steamworks-command.exe ugc unsubscribe --id <UGC_ID>
./steamworks-command.exe --appid <APP_ID> ugc subscribed
```

### 用户操作

```bash
# 获取当前用户信息
./steamworks-command.exe user info
# 查询好友列表
./steamworks-command.exe user friends
```

### 应用操作

```bash
# 获取应用基本信息
./steamworks-command.exe --appid <APP_ID> app info
```

## 开发贡献

欢迎通过 Issue 提交问题或通过 Pull Request 贡献代码：

1. Fork 仓库
2. 创建特性分支 (`git checkout -b feature/awesome-feature`)
3. 提交修改 (`git commit -am 'Add awesome feature'`)
4. 推送分支 (`git push origin feature/awesome-feature`)
5. 创建 Pull Request

## 注意事项

- 运行该工具前需要打开 Steam
- 该工具仅支持 Windows 操作系统
- 该工具仅支持 Steamworks API 的部分功能
- 该工具需要在 steam_api64.dll 文件（build 时会自动打包到目标目录）所在目录下运行
- 本项目使用 Steamworks-rs 版本为最新 master(当前日期:2025/04/06)，并且有修改（修复了 file_size 显示问题，及在最新 steam sdk 上无法运行的问题）
- Steam SDK 下载地址：https://partner.steamgames.com/doc/sdk

## 许可证

本项目基于 [MIT 许可证](LICENSE) 发布

## 技术支持

遇到问题请提交 [GitHub Issue](https://gitcode.com/zeyl/steamworks-command/issues)

## 鸣谢

- [Steamworks-rs](https://github.com/Noxime/steamworks-rs)
