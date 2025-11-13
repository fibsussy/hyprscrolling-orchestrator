use serde::{Deserialize, Serialize};

/// Tuple for coordinates / sizes: (x, y) and (w, h)
pub type Cords = (i32, i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRef {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: Cords,          // (x, y)
    pub size: Cords,        // (w, h)
    pub workspace: WorkspaceRef,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: i64,
    pub class: String,
    pub title: String,
    #[serde(default)]
    pub fullscreen: Option<i64>,
    #[serde(default)]
    pub xwayland: Option<bool>,
    #[serde(default)]
    pub grouped: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWorkspace {
    pub id: i64,
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub monitor: String,
    pub monitor_id: i64,
    pub windows: i64,
    pub has_fullscreen: bool,
    pub last_window: Option<String>,
    pub last_window_title: Option<String>,
    pub is_persistent: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
