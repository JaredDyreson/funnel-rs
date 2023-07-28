use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Track {
    pub uri: String,
}
#[derive(Deserialize, Serialize)]
pub struct Items {
    pub items: Vec<Track>
}

