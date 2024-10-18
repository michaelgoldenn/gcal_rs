use serde::{Deserialize, Serialize};
use url::Url;

use super::{ClientResult, QueryParams, Sendable};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub picture: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd: Option<String>,

    #[serde(skip)]
    query_string: QueryParams,
}

impl Sendable for UserInfo {
    fn path(&self, _action: Option<String>) -> String {
        String::new()
    }

    fn query(&self) -> QueryParams {
        self.query_string.clone()
    }

    fn url(&self, action: Option<String>) -> ClientResult<Url> {
        Ok(Url::parse_with_params(
            &format!(
                "{}/{}",
                "https://www.googleapis.com/oauth2/v2/userinfo",
                self.path(action)
            ),
            self.query(),
        )?)
    }
}
