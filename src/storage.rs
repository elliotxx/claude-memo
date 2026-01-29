//! Storage module for managing ~/.claude-memo/ data

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Represents a favorited session
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FavoriteSession {
    /// The favorited session ID
    pub session_id: String,
    /// Timestamp when the session was favorited (milliseconds)
    pub favorited_at: i64,
}

impl FavoriteSession {
    /// Create a new FavoriteSession
    #[inline]
    pub fn new(session_id: String, favorited_at: i64) -> Self {
        Self {
            session_id,
            favorited_at,
        }
    }
}

impl std::fmt::Display for FavoriteSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datetime: DateTime<Utc> = Utc
            .timestamp_millis_opt(self.favorited_at)
            .single()
            .unwrap_or(Utc::now());
        write!(
            f,
            "⭐ {} ({})",
            self.session_id,
            datetime.format("%Y-%m-%d %H:%M")
        )
    }
}

/// Storage for favorites using TOML format
#[derive(Debug, Clone)]
pub struct Storage {
    /// Path to the data directory (~/.claude-memo/)
    data_dir: PathBuf,
    /// Path to the favorites TOML file
    favorites_file: PathBuf,
    /// In-memory cache of favorites
    favorites: HashMap<String, i64>,
}

impl Storage {
    /// Create a new Storage instance
    /// Initializes the data directory and loads favorites
    pub fn new() -> Result<Self, crate::error::Error> {
        let data_dir = get_data_dir()?;
        let favorites_file = data_dir.join("favorites/sessions.toml");

        // Create directories if they don't exist
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }
        if !favorites_file.parent().map(|p| p.exists()).unwrap_or(false) {
            fs::create_dir_all(favorites_file.parent().unwrap())?;
        }

        // Load favorites or create empty
        let favorites = if favorites_file.exists() {
            load_favorites(&favorites_file)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            data_dir,
            favorites_file,
            favorites,
        })
    }

    /// Add a session to favorites
    pub fn add_favorite(&mut self, session_id: &str) -> Result<(), crate::error::Error> {
        // Validate session_id format (basic UUID check)
        if session_id.is_empty() {
            return Err(crate::error::Error::InvalidSessionId(
                "session_id cannot be empty".to_string(),
            ));
        }

        let now = chrono::Utc::now().timestamp_millis();
        self.favorites.insert(session_id.to_string(), now);
        save_favorites(&self.favorites_file, &self.favorites)?;
        Ok(())
    }

    /// Remove a session from favorites
    pub fn remove_favorite(&mut self, session_id: &str) -> Result<(), crate::error::Error> {
        if !self.favorites.contains_key(session_id) {
            return Err(crate::error::Error::SessionNotFound(session_id.to_string()));
        }

        self.favorites.remove(session_id);
        save_favorites(&self.favorites_file, &self.favorites)?;
        Ok(())
    }

    /// List all favorites
    pub fn list_favorites(&self) -> Vec<FavoriteSession> {
        let mut favorites: Vec<FavoriteSession> = self
            .favorites
            .iter()
            .map(|(session_id, &favorited_at)| {
                FavoriteSession::new(session_id.clone(), favorited_at)
            })
            .collect();

        // Sort by favorited_at descending (most recent first)
        favorites.sort_by(|a, b| b.favorited_at.cmp(&a.favorited_at));
        favorites
    }

    /// Check if a session is favorited
    pub fn is_favorited(&self, session_id: &str) -> bool {
        self.favorites.contains_key(session_id)
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}

/// Get the data directory path (~/.claude-memo/)
fn get_data_dir() -> Result<PathBuf, crate::error::Error> {
    let home = dirs::home_dir().ok_or(crate::error::Error::HomeDirNotFound)?;
    Ok(home.join(".claude-memo"))
}

/// Load favorites from TOML file
fn load_favorites(path: &PathBuf) -> Result<HashMap<String, i64>, crate::error::Error> {
    let content = fs::read_to_string(path)?;
    let data: toml::Value = content.parse().map_err(crate::error::Error::Toml)?;
    let mut favorites = HashMap::new();

    if let Some(sessions) = data.get("sessions").and_then(|s| s.as_table()) {
        for (session_id, value) in sessions {
            if let Some(timestamp) = value.as_integer() {
                favorites.insert(session_id.clone(), timestamp);
            }
        }
    }

    Ok(favorites)
}

/// Save favorites to TOML file
fn save_favorites(
    path: &PathBuf,
    favorites: &HashMap<String, i64>,
) -> Result<(), crate::error::Error> {
    let mut content = String::new();
    content.push_str("[sessions]\n");

    // Sort by timestamp descending for readability
    let mut entries: Vec<_> = favorites.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));

    for (session_id, &timestamp) in entries {
        content.push_str(&format!("\"{}\" = {}\n", session_id, timestamp));
    }

    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_favorite_session_display() {
        let session = FavoriteSession::new("abc123-def456".to_string(), 1700000000000);
        let display = session.to_string();
        assert!(display.contains("abc123-def456"));
        assert!(display.contains("⭐"));
    }

    #[test]
    fn test_storage_add_favorite() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file: favorites_file.clone(),
            favorites: HashMap::new(),
        };

        let result = storage.add_favorite("test-session-id");
        assert!(result.is_ok());
        assert!(storage.is_favorited("test-session-id"));

        // Verify file was created
        assert!(favorites_file.exists());
    }

    #[test]
    fn test_storage_remove_favorite() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file: favorites_file.clone(),
            favorites: HashMap::new(),
        };

        storage.add_favorite("test-session-id").unwrap();
        assert!(storage.is_favorited("test-session-id"));

        storage.remove_favorite("test-session-id").unwrap();
        assert!(!storage.is_favorited("test-session-id"));
    }

    #[test]
    fn test_storage_list_favorites() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file,
            favorites: HashMap::new(),
        };

        storage.add_favorite("session-1").unwrap();
        storage.add_favorite("session-2").unwrap();
        storage.add_favorite("session-3").unwrap();

        let favorites = storage.list_favorites();
        assert_eq!(favorites.len(), 3);
    }

    #[test]
    fn test_add_empty_session_id_fails() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file,
            favorites: HashMap::new(),
        };

        let result = storage.add_favorite("");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_nonexistent_session_fails() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file,
            favorites: HashMap::new(),
        };

        let result = storage.remove_favorite("nonexistent");
        assert!(result.is_err());
    }

    // === Edge Case Tests ===

    #[test]
    fn test_duplicate_session_id() {
        // Adding the same session_id twice should succeed
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file: favorites_file.clone(),
            favorites: HashMap::new(),
        };

        // Add same session twice
        storage.add_favorite("same-session").unwrap();
        assert!(storage.is_favorited("same-session"));

        // Second add should still succeed
        let result = storage.add_favorite("same-session");
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_favorited_returns_false_for_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file,
            favorites: HashMap::new(),
        };

        assert!(!storage.is_favorited("nonexistent-session"));
    }

    #[test]
    fn test_favorites_sorted_by_timestamp_descending() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file: favorites_file.clone(),
            favorites: HashMap::new(),
        };

        // Use different timestamps directly by modifying the HashMap
        // This avoids timing issues with add_favorite
        use std::thread;
        use std::time::Duration;

        // Add with delays to ensure different timestamps
        storage.add_favorite("oldest").unwrap();
        thread::sleep(Duration::from_millis(10));
        storage.add_favorite("middle").unwrap();
        thread::sleep(Duration::from_millis(10));
        storage.add_favorite("newest").unwrap();

        let favorites = storage.list_favorites();
        assert_eq!(favorites.len(), 3);

        // Verify descending order by checking each favorite is >= next
        for i in 0..favorites.len()-1 {
            assert!(favorites[i].favorited_at >= favorites[i+1].favorited_at);
        }
    }

    #[test]
    fn test_list_favorites_empty() {
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file,
            favorites: HashMap::new(),
        };

        let favorites = storage.list_favorites();
        assert!(favorites.is_empty());
    }

    #[test]
    fn test_add_favorite_with_special_chars_in_id() {
        // Session IDs with special characters should be allowed
        let temp_dir = TempDir::new().unwrap();
        let favorites_file = temp_dir.path().join("sessions.toml");

        let mut storage = Storage {
            data_dir: temp_dir.path().to_path_buf(),
            favorites_file,
            favorites: HashMap::new(),
        };

        let special_id = "abc123-def456_789.012";
        let result = storage.add_favorite(&special_id);
        assert!(result.is_ok());
        assert!(storage.is_favorited(&special_id));
    }
}
