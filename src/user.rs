pub struct User {
    token: String,
    id: String,
}

impl User {
    /// Constructor for a Spotify user
    #[allow(dead_code)]
    fn new(token: String) -> Self {
        Self {
            token,
            id: "".to_string(),
        }
    }
}
