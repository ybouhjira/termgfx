//! Persistent storage for Studio favorites and history
//!
//! Saves configuration to ~/.config/termgfx/studio.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Maximum number of favorites to store
const MAX_FAVORITES: usize = 100;
/// Maximum number of history entries to store
const MAX_HISTORY: usize = 10;

/// A saved favorite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub name: String,
    pub component: String,
    pub params: HashMap<String, String>,
    pub created_at: u64,
}

/// A history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub component: String,
    pub params: HashMap<String, String>,
    pub timestamp: u64,
}

/// Persistent storage data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudioStorage {
    pub favorites: Vec<Favorite>,
    pub history: Vec<HistoryEntry>,
}

impl StudioStorage {
    /// Get the config directory path
    fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("termgfx")
    }

    /// Get the storage file path
    fn storage_path() -> PathBuf {
        Self::config_dir().join("studio.json")
    }

    /// Load storage from disk
    pub fn load() -> Self {
        let path = Self::storage_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    /// Save storage to disk using atomic write (temp file + rename)
    pub fn save(&self) -> std::io::Result<()> {
        let dir = Self::config_dir();
        fs::create_dir_all(&dir)?;
        let path = Self::storage_path();
        let temp_path = path.with_extension("json.tmp");
        let content = serde_json::to_string_pretty(self)?;
        // Write to temp file first, then rename for atomic operation
        fs::write(&temp_path, content)?;
        fs::rename(&temp_path, path)
    }

    /// Add a favorite
    pub fn add_favorite(
        &mut self,
        name: String,
        component: String,
        params: HashMap<String, String>,
    ) {
        let created_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Remove existing favorite with same name
        self.favorites.retain(|f| f.name != name);

        self.favorites.insert(
            0,
            Favorite {
                name,
                component,
                params,
                created_at,
            },
        );

        // Keep only last MAX_FAVORITES entries
        self.favorites.truncate(MAX_FAVORITES);
    }

    /// Remove a favorite by name
    pub fn remove_favorite(&mut self, name: &str) {
        self.favorites.retain(|f| f.name != name);
    }

    /// Add a history entry
    pub fn add_history(&mut self, component: String, params: HashMap<String, String>) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Don't add duplicate consecutive entries
        if let Some(last) = self.history.first() {
            if last.component == component && last.params == params {
                return;
            }
        }

        self.history.insert(
            0,
            HistoryEntry {
                component,
                params,
                timestamp,
            },
        );

        // Keep only last MAX_HISTORY entries
        self.history.truncate(MAX_HISTORY);
    }

    /// Get relative time string (e.g., "2m ago", "1h ago")
    #[allow(dead_code)]
    pub fn relative_time(timestamp: u64) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let diff = now.saturating_sub(timestamp);

        if diff < 60 {
            "just now".to_string()
        } else if diff < 3600 {
            format!("{}m ago", diff / 60)
        } else if diff < 86400 {
            format!("{}h ago", diff / 3600)
        } else {
            format!("{}d ago", diff / 86400)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_default() {
        let storage = StudioStorage::default();
        assert!(storage.favorites.is_empty());
        assert!(storage.history.is_empty());
    }

    #[test]
    fn test_add_favorite() {
        let mut storage = StudioStorage::default();
        let mut params = HashMap::new();
        params.insert("message".to_string(), "Hello".to_string());

        storage.add_favorite("My Box".to_string(), "box".to_string(), params.clone());

        assert_eq!(storage.favorites.len(), 1);
        assert_eq!(storage.favorites[0].name, "My Box");
        assert_eq!(storage.favorites[0].component, "box");
    }

    #[test]
    fn test_add_favorite_replaces_same_name() {
        let mut storage = StudioStorage::default();
        let mut params1 = HashMap::new();
        params1.insert("message".to_string(), "Hello".to_string());

        let mut params2 = HashMap::new();
        params2.insert("message".to_string(), "World".to_string());

        storage.add_favorite("My Box".to_string(), "box".to_string(), params1);
        storage.add_favorite("My Box".to_string(), "box".to_string(), params2.clone());

        assert_eq!(storage.favorites.len(), 1);
        assert_eq!(
            storage.favorites[0].params.get("message"),
            Some(&"World".to_string())
        );
    }

    #[test]
    fn test_remove_favorite() {
        let mut storage = StudioStorage::default();
        storage.add_favorite("Test".to_string(), "box".to_string(), HashMap::new());

        storage.remove_favorite("Test");
        assert!(storage.favorites.is_empty());
    }

    #[test]
    fn test_add_history() {
        let mut storage = StudioStorage::default();
        let params = HashMap::new();

        storage.add_history("box".to_string(), params.clone());
        storage.add_history("chart".to_string(), params.clone());

        assert_eq!(storage.history.len(), 2);
        assert_eq!(storage.history[0].component, "chart"); // Most recent first
        assert_eq!(storage.history[1].component, "box");
    }

    #[test]
    fn test_history_no_duplicates() {
        let mut storage = StudioStorage::default();
        let params = HashMap::new();

        storage.add_history("box".to_string(), params.clone());
        storage.add_history("box".to_string(), params.clone()); // Duplicate

        assert_eq!(storage.history.len(), 1);
    }

    #[test]
    fn test_history_max_10() {
        let mut storage = StudioStorage::default();

        for i in 0..15 {
            let mut params = HashMap::new();
            params.insert("id".to_string(), i.to_string());
            storage.add_history("box".to_string(), params);
        }

        assert_eq!(storage.history.len(), 10);
    }

    #[test]
    fn test_relative_time() {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(StudioStorage::relative_time(now), "just now");
        assert_eq!(StudioStorage::relative_time(now - 120), "2m ago");
        assert_eq!(StudioStorage::relative_time(now - 3700), "1h ago");
        assert_eq!(StudioStorage::relative_time(now - 90000), "1d ago");
    }
}
