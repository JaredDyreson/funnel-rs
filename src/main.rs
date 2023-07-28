mod playlist;
mod response_structures;
mod token;
mod tracks;
mod user;

#[tokio::main]
async fn main() {
    let mut client = reqwest::Client::builder().build().unwrap();
    let tok = serde_json::from_str::<token::Token>(
        &std::fs::read_to_string("inputs/credentials.json").unwrap(),
    )
    .unwrap();

    let play = playlist::Playlist {
        name: None,
        id: Some("5SKUQothH5U87zt5bg8XLP".to_string()),
        tracks: std::collections::BTreeSet::new(),
    };

    let retrieve_information_url =
        format!("https://api.spotify.com/v1/playlists/{}", play.id.unwrap());

    let params = [("fields", "tracks.items(track(uri))")];

    let new_playlist_url = format!("https://api.spotify.com/v1/users/{}/playlists", tok.user_id);

    let response = client
        .post(new_playlist_url)
        .bearer_auth(tok.oauth_token)
        .body(
            serde_json::to_string(&response_structures::PlaylistRequest {
                name: "__high__ | cloned".to_string(),
                description: "Hello World!".to_string(),
                public: true,
            })
            .unwrap(),
        )
        .send()
        .await
        .unwrap();

    println!("{}", response.text().await.unwrap());
}
