use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

mod client;
pub use client::*;

/// Calendar List, the normal way to get at the list of calendars available.
pub mod list;
pub use list::*;

mod types;
pub use types::*;

use super::*;

/* Google Calendar API: https://developers.google.com/calendar/api/v3/reference/calendars#resource */

/// Calendar is a single calendar.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    #[serde(default = "default_kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub id: String,
    pub etag: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_properties: Option<ConferenceProperties>,
}
impl Sendable for Calendar {
    fn path(&self, _action: Option<String>) -> String {
        format!("calendars/{}", self.id)
    }

    fn query(&self) -> QueryParams {
        Default::default()
    }
}
fn default_kind() -> Option<String> {
    Some("calendar#calendar".to_string())
}

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