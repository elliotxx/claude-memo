//! CLI module for command-line interface

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Claude Code 会话记录管理工具
#[derive(Parser, Debug)]
#[command(name = "claude-memo")]
#[command(author = "yym")]
#[command(version = "0.1.0")]
#[command(about = "Claude Code 会话记录管理工具", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// CLI 子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 解析并显示历史记录（调试用）
    #[command(name = "parse")]
    Parse(ParseArgs),

    /// 全文搜索会话记录
    #[command(name = "search")]
    Search(SearchArgs),

    /// 添加收藏
    #[command(name = "favorite")]
    Favorite(AddFavoriteArgs),

    /// 取消收藏
    #[command(name = "unfavorite")]
    Unfavorite(RemoveFavoriteArgs),

    /// 列出所有收藏
    #[command(name = "favorites")]
    Favorites(ListFavoriteArgs),
}

/// Parse 命令参数
#[derive(Parser, Debug)]
pub struct ParseArgs {
    /// JSON 格式输出
    #[arg(long = "json")]
    pub json: bool,

    /// 限制显示数量
    #[arg(long = "limit", short = 'n')]
    pub limit: Option<usize>,
}

/// Search 命令参数
#[derive(Parser, Debug)]
pub struct SearchArgs {
    /// 搜索关键词
    pub keyword: String,

    /// JSON 格式输出
    #[arg(long = "json")]
    pub json: bool,

    /// 限制结果数量 (默认: 20)
    #[arg(long = "limit", short = 'n')]
    pub limit: Option<usize>,
}

/// 添加收藏参数
#[derive(Parser, Debug)]
pub struct AddFavoriteArgs {
    /// 会话 ID
    pub session_id: String,
}

/// 取消收藏参数
#[derive(Parser, Debug)]
pub struct RemoveFavoriteArgs {
    /// 会话 ID
    pub session_id: String,
}

/// 列出收藏参数
#[derive(Parser, Debug)]
pub struct ListFavoriteArgs {
    /// JSON 格式输出
    #[arg(long = "json")]
    pub json: bool,
}

/// 获取历史文件路径
pub fn get_history_path() -> PathBuf {
    // Check CLAUDE_HISTORY env var first
    if let Ok(path) = std::env::var("CLAUDE_HISTORY") {
        return PathBuf::from(path);
    }

    // Default to ~/.claude/history.jsonl
    let home = dirs::home_dir().unwrap_or(PathBuf::from("."));
    home.join(".claude/history.jsonl")
}
