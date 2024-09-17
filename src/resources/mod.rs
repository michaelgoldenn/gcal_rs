use serde::{Deserialize, Serialize};

/// Calendar List, the normal way to get at the list of calendars available.
pub mod calendar_list;
pub use calendar_list::*;

pub mod calendar;
pub use calendar::*;

pub mod conference_properties;
pub use conference_properties::*;

/// Events, the method you will work with most events in a single calendar.
pub mod events;
pub use events::*;

use super::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SendUpdates {
    #[default]
    All,
    ExternalOnly,
    None,
}
impl SendUpdates {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::ExternalOnly => "externalOnly",
            Self::None => "none",
        }
    }
}
impl ToString for SendUpdates {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CalendarAccessRole {
    #[default]
    Owner,
    Reader,
    Writer,
    FreeBusyReader,
}
impl CalendarAccessRole {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Reader => "reader",
            Self::Writer => "writer",
            Self::FreeBusyReader => "freeBusyReader",
        }
    }
}
impl ToString for CalendarAccessRole {
    fn to_string(&self) -> String {
        self.to_str().into()
    }
}
