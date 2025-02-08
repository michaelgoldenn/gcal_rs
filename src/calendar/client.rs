use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use super::{CalendarAccessRole, CalendarList, CalendarListItem, ClientResult, GCalClient};

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarListOptions {
    pub max_results: Option<u32>,      // Default: 100, Max: 250
    pub min_access_role: Option<MinAccessRole>,
    pub page_token: Option<String>,
    pub show_deleted: Option<bool>,    // Default: false
    pub show_hidden: Option<bool>,     // Default: false
    pub sync_token: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MinAccessRole {
    FreeBusyReader,  // Can read free/busy information
    Owner,          // Can read and modify events and access control lists
    Reader,         // Can read non-private events
    Writer,         // Can read and modify events
}
// Implementation for string conversion
impl From<&str> for MinAccessRole {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "freebusyreader" => MinAccessRole::FreeBusyReader,
            "owner" => MinAccessRole::Owner,
            "reader" => MinAccessRole::Reader,
            "writer" => MinAccessRole::Writer,
            _ => panic!("Invalid access role"),  // Or handle error differently
        }
    }
}

// Optional: Implementation for serialization
impl ToString for MinAccessRole {
    fn to_string(&self) -> String {
        match self {
            MinAccessRole::FreeBusyReader => "freeBusyReader".to_string(),
            MinAccessRole::Owner => "owner".to_string(),
            MinAccessRole::Reader => "reader".to_string(),
            MinAccessRole::Writer => "writer".to_string(),
        }
    }
}

/// CalendarListClient is the method of accessing the calendar list. You must provide it with a
/// Google Calendar client.
#[derive(Debug, Clone)]
pub struct CalendarListClient(Arc<GCalClient>);

impl CalendarListClient {
    /// Construct a CalendarListClient. Requires a Google Calendar Client.
    pub fn new(client: Arc<GCalClient>) -> Self {
        Self(client)
    }

    pub async fn list(
        &self,
        options: Option<CalendarListOptions>,
    ) -> ClientResult<Vec<CalendarListItem>> {
        let mut cl = CalendarList::default();
        
        if let Some(opts) = options {
            // Convert the options to query parameters automatically
            let query_params = serde_qs::to_string(&opts)?;
            for (key, value) in serde_qs::from_str::<Vec<(String, String)>>(&query_params)? {
                cl.add_query(key, value);
            }
        }
    
        Ok(self
            .0
            .get(None, cl)
            .await?
            .json::<CalendarList>()
            .await?
            .items)
    }
}
