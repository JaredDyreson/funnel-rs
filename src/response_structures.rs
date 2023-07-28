use serde::{Deserialize, Serialize};
use crate::tracks;

#[derive(Deserialize, Serialize)]
pub struct PlaylistRequest {
    pub name: String,
    pub description: String,
    pub public: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TracksResponse {
    pub tracks: tracks::Items,
}

#[derive(Deserialize, Serialize)]
pub struct AddTrackRequest {
    pub uris: Vec<String>,
    pub position: u32,
}

