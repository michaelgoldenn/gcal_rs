use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

mod client;
pub use client::EventClient;

pub mod types;
use types::*;

use super::*;

/* Google API Source: https://developers.google.com/calendar/api/v3/reference/events#resource */

/// Events is a listing of events on a per-page basis.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Events {
    #[serde(
        default = "default_events_kind",
        skip_serializing_if = "Option::is_none"
    )]
    pub kind: Option<String>,
    pub etag: String,
    pub summary: String,
    pub description: String,
    pub updated: String,
    pub time_zone: String,
    pub access_role: CalendarAccessRole,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_reminders: Vec<DefaultReminder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}

/// Event is a single event.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(
        default = "default_event_kind",
        skip_serializing_if = "Option::is_none"
    )]
    pub kind: Option<String>,
    pub id: String,
    pub summary: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendees>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EventAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees_omitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<EventConferenceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<EventCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<EventCalendarDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_unspecified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<EventExtendedProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gadget: Option<EventGadget>,
    #[serde(
        rename = "guestsCanInviteOthers",
        default = "default_true",
        skip_serializing_if = "Option::is_none"
    )]
    pub guests_invite_others: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_modify: Option<bool>,
    #[serde(default = "default_true", skip_serializing_if = "Option::is_none")]
    pub guests_can_see_other_guests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangout_link: Option<String>,
    #[serde(rename = "iCalUID", skip_serializing_if = "Option::is_none")]
    pub ical_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<EventOrganizer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<EventCalendarDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<BTreeSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<EventReminder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<EventCalendarDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<EventTransparency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<EventVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_location: Option<EventWorkingLocation>,

    #[serde(skip)]
    pub calendar_id: Option<String>,
    #[serde(skip)]
    query_string: QueryParams,
}

impl Sendable for Event {
    fn path(&self, action: Option<String>) -> String {
        format!(
            "calendars/{}/events{}{}",
            self.calendar_id.clone().unwrap(),
            self.id,
            // .clone()
            // .map_or_else(String::new, |x| format!("/{}", x)),
            action.map_or_else(String::new, |x| format!("/{}", x))
        )
    }

    fn query(&self) -> QueryParams {
        self.query_string.clone()
    }
}

impl Events {
    pub fn add_calendar(&mut self, calendar_id: String) {
        self.items.iter_mut().for_each(|e| {
            e.calendar_id = Some(calendar_id.clone());
        });
    }
}

impl Event {
    pub fn add_query(&mut self, key: String, value: String) {
        self.query_string.insert(key, value);
    }
}

fn default_event_kind() -> Option<String> {
    Some("calendar#event".to_string())
}
fn default_events_kind() -> Option<String> {
    Some("calendar#events".to_string())
}
fn default_true() -> Option<bool> {
    Some(true)
}
