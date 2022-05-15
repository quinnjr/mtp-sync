use std::collections::HashMap;

use serde::{Deserialize};

use crate::track::Track;
use crate::playlist::Playlist;

/// iTunes Library structure
#[derive(Deserialize)]
pub struct Library {
    #[serde(rename(deserialize = "Major Version"))]
    major_version: i8,
    #[serde(rename(deserialize = "Minor Version"))]
    minor_version: i8,
    #[serde(rename(deserialize = "Application Version"))]
    application_version: String,
    #[serde(rename(deserialize = "Date"))]
    date: String,
    #[serde(rename(deserialize = "Features"))]
    features: i8,
    #[serde(rename(deserialize = "Show Content Ratings"))]
    show_content_ratings: bool,
    #[serde(rename(deserialize = "Library Persistent ID"))]
    library_persistent_id: String,
    #[serde(rename(deserialize = "Tracks"))]
    tracks: HashMap<String, Track>,
    #[serde(rename(deserialize = "Playlists"))]
    playlists: Vec<Playlist>
}
