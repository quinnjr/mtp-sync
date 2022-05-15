use serde::{Deserialize};

/// iTunes PLaylist struct
#[derive(Deserialize)]
pub struct Playlist {
    #[serde(rename(deserialize = "Master"))]
    master: Option<bool>,
    #[serde(rename(deserialize = "Playlist ID"))]
    playlist_id: isize,
    #[serde(rename(deserialize = "Playlist Persistent ID"))]
    playlist_persistent_id: String,
    #[serde(rename(deserialize = "All Items"))]
    all_items: bool,
    #[serde(rename(deserialize = "Visible"))]
    visible: Option<bool>,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Playlist Items"))]
    playlist_items: Option<Vec<PlaylistItem>>
}

#[derive(Deserialize)]
pub struct PlaylistItem {
    #[serde(rename(deserialize = "Track ID"))]
    track_id: isize
}
