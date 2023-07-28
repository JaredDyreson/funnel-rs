use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use chrono::TimeZone;

fn deserialize_datetime_from_string<'de, D>(
    deserializer: D,
) -> Result<chrono::DateTime<chrono::Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    chrono::Local
        .datetime_from_str(&s, "%m/%d/%Y %H:%M:%S")
        .map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub user_id: String,
    pub username: String,
    pub oauth_token: String,
    #[serde(deserialize_with = "deserialize_datetime_from_string")]
    time_expires: chrono::DateTime<chrono::Local>,
}

impl Token {
    /// Check if the token is still valid
    #[allow(dead_code)]
    fn expired(&self) -> bool {
        chrono::Local::now() > self.time_expires
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn yield_token() -> Token {
        serde_json::from_str::<Token>(&std::fs::read_to_string("inputs/credentials.json").unwrap())
            .unwrap()
    }

    #[test]
    fn test_sample_file() {
        yield_token();
    }

    #[test]
    fn test_validity() {
        assert_eq!(yield_token().expired(), true);
    }
}
