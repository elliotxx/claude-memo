//! claude-memo: Claude Code session record management tool
//!
//! # Usage
//!
//! ```
//! claude-memo parse          # 解析并显示历史记录
//! claude-memo search "关键词" # 全文搜索
//! claude-memo mark <session-id>  # 添加收藏
//! claude-memo unmark <session-id> # 取消收藏
//! claude-memo marks         # 列出所有收藏
//! ```

use clap::Parser;
use claude_memo::cli::{get_history_path, Cli, Commands};
use claude_memo::parser::parse_history_file;
use claude_memo::storage::Storage;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Handle commands
    match &cli.command {
        Commands::Parse(args) => {
            if let Err(e) = handle_parse(args.json, args.limit) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Commands::Search(args) => {
            if let Err(e) = handle_search(&args.keyword, args.limit, args.json) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Commands::Mark(args) => {
            if let Err(e) = handle_mark_add(&args.session_id) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Commands::Unmark(args) => {
            if let Err(e) = handle_mark_remove(&args.session_id) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Commands::Marks(args) => {
            if let Err(e) = handle_mark_list(args.json) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
    }
}

/// 处理 parse 命令
fn handle_parse(json: bool, limit: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_history_path();

    if !path.exists() {
        eprintln!("Error: File not found: {}", path.display());
        process::exit(3);
    }

    let records = parse_history_file(&path)?;

    let records: Vec<_> = match limit {
        Some(n) => records.into_iter().rev().take(n).collect(),
        None => records.into_iter().rev().collect(),
    };

    if json {
        // JSON output
        let output: Vec<serde_json::Value> = records
            .iter()
            .map(|r| {
                serde_json::json!({
                    "display": r.display,
                    "timestamp": r.timestamp,
                    "project": r.project,
                    "session_id": r.session_id
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        // Text output
        for record in records {
            println!("{record}");
        }
    }

    Ok(())
}

/// 处理 search 命令
fn handle_search(
    keyword: &str,
    limit: Option<usize>,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use claude_memo::indexer::Indexer;
    use claude_memo::search::Search;

    let path = get_history_path();

    if !path.exists() {
        eprintln!("Error: File not found: {}", path.display());
        process::exit(3);
    }

    // Parse history file and build index
    let records = parse_history_file(&path)?;

    // Build or update the FTS5 index
    let indexer = Indexer::new()?;
    indexer.build_index(&records)?;

    // Search using FTS5
    let search = Search::new()?;
    let results = search.search(keyword, limit)?;

    if results.is_empty() {
        println!("No results found for: {keyword}");
        return Ok(());
    }

    if json {
        let output: Vec<serde_json::Value> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "display": r.record.display,
                    "timestamp": r.record.timestamp,
                    "project": r.record.project,
                    "session_id": r.record.session_id,
                    "score": r.score
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        for result in results {
            println!("{}", result.record);
        }
    }

    Ok(())
}

/// 处理 mark add 命令
fn handle_mark_add(session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = Storage::new()?;
    storage.add_favorite(session_id)?;
    println!("✅ Added {session_id} to marks");
    Ok(())
}

/// 处理 mark remove 命令
fn handle_mark_remove(session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = Storage::new()?;
    storage.remove_favorite(session_id)?;
    println!("✅ Removed {session_id} from marks");
    Ok(())
}

/// 处理 marks list 命令
fn handle_mark_list(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    use claude_memo::storage::FavoriteWithDetails;

    let storage = Storage::new()?;
    let history_path = get_history_path();

    // Get favorites enriched with session details from history
    let favorites: Vec<FavoriteWithDetails> = storage.list_favorites_with_details(&history_path)?;

    if favorites.is_empty() {
        println!("No marks yet.");
        return Ok(());
    }

    if json {
        let output: Vec<serde_json::Value> = favorites
            .iter()
            .map(|f| {
                serde_json::json!({
                    "session_id": f.session_id,
                    "favorited_at": f.favorited_at,
                    "display": f.display,
                    "project": f.project,
                    "timestamp": f.session_timestamp
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        for favorite in favorites {
            println!("{favorite}");
        }
    }

    Ok(())
}
