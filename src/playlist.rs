use serde::{Deserialize, Serialize};

use crate::{token, tracks};
use crate::response_structures::AddTrackRequest;

#[derive(Deserialize, Serialize)]
pub struct Playlist {
    #[serde(skip)]
    pub name: Option<String>,
    #[serde(skip)]
    pub id: Option<String>,
    pub tracks: std::collections::BTreeSet<tracks::Track>,
}

impl Playlist {
    #[allow(dead_code)]
    fn merge(&self, rhs: &Self) -> std::collections::BTreeSet<tracks::Track> {
        self.tracks.union(&rhs.tracks).cloned().collect()
    }

    #[allow(dead_code)]
    async fn make_remote_playlist(
        &self,
        token: &token::Token,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        Ok(reqwest::Client::builder()
            .build()?
            .post(format!(
                "https://api.spotify.com/v1/users/{}/playlists",
                token.user_id
            ))
            .bearer_auth(token.oauth_token.clone())
            .body(serde_json::to_string(&self)?)
            .send()
            .await?)
    }

    #[allow(dead_code)]
    async fn add_remote_ids(
        &self,
        token: &token::Token,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        Ok(reqwest::Client::builder()
            .build()?
            .post(format!(
                "https://api.spotify.com/v1/playlists/{}/tracks",
                self.id.clone().unwrap()
            ))
            .bearer_auth(token.oauth_token.clone())
            .body(serde_json::to_string(&AddTrackRequest {
                uris: self.tracks.iter().map(|value| value.uri.clone()).collect(),
                position: 0_u32,
            })?)
            .send()
            .await?)
    }

    #[allow(dead_code)]
    /// Get readings from the API and generate an updated instance of the
    /// object with the newly populated fields
    fn from_api(&self) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    //#[allow(dead_code)]
    //fn from_api_mut(self) -> Self {
    //match self.from_api() {
    //Some(retrieved) => retrieved,
    //None => self,
    //}
    //}
}
