use serde::{Deserialize};

/// A Track in the iTunes Library
#[derive(Deserialize)]
pub struct Track {
    #[serde(rename(deserialize = "Track ID"))]
    track_id: isize,
    #[serde(rename(deserialize = "Size"))]
    size: isize,
    #[serde(rename(deserialize = "Total Time"))]
    total_time: isize,
    #[serde(rename(deserialize = "Disc Number"))]
    disc_number: Option<isize>,
    #[serde(rename(deserialize = "Disc Count"))]
    disc_count: Option<isize>,
    #[serde(rename(deserialize = "Track Number"))]
    track_number: Option<isize>,
    #[serde(rename(deserialize = "Track Count"))]
    track_count: Option<isize>,
    #[serde(rename(deserialize = "Year"))]
    year: Option<isize>,
    #[serde(rename(deserialize = "Date Modified"))]
    date_modified: String,
    #[serde(rename(deserialize = "Date Added"))]
    date_added: String,
    #[serde(rename(deserialize = "Bit Rate"))]
    bit_rate: Option<isize>,
    #[serde(rename(deserialize = "Sample Rate"))]
    sample_rate: Option<isize>,
    #[serde(rename(deserialize = "Release Date"))]
    release_date: Option<String>,
    #[serde(rename(deserialize = "Artwork Count"))]
    artwork_count: Option<isize>,
    #[serde(rename(deserialize = "Persistent ID"))]
    persistent_id: String,
    #[serde(rename(deserialize = "Track Type"))]
    track_type: String,
    #[serde(rename(deserialize = "Protected"))]
    protected: Option<bool>,
    #[serde(rename(deserialize = "Purchased"))]
    purchased: Option<bool>,
    #[serde(rename(deserialize = "File Folder Count"))]
    file_folder_count: Option<isize>,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Artist"))]
    artist: Option<String>,
    #[serde(rename(deserialize = "Album Artist"))]
    album_artist: Option<String>,
    #[serde(rename(deserialize = "Composer"))]
    composer: Option<String>,
    #[serde(rename(deserialize = "Album"))]
    album: Option<String>,
    #[serde(rename(deserialize = "Genre"))]
    genre: Option<String>,
    #[serde(rename(deserialize = "Kind"))]
    kind: String,
    #[serde(rename(deserialize = "Sort Name"))]
    sort_name: Option<String>,
    #[serde(rename(deserialize = "Sort Album"))]
    sort_album: Option<String>,
    #[serde(rename(deserialize = "Sort Artist"))]
    sort_artist: Option<String>,
    #[serde(rename(deserialize = "Location"))]
    location: Option<String>
}
