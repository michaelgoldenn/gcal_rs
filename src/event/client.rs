use std::{sync::Arc, default::Default};
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use super::{ClientResult, Event, Events, GCalClient, SendUpdates};

/// EventClient is the method of managing events from a specific calendar. Requires a Google
/// Calendar client.
#[derive(Debug, Clone)]
pub struct EventClient(Arc<GCalClient>);

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListOptions {
    pub event_types: Option<Vec<EventTypes>>,
    pub ical_uid: Option<String>,
    pub max_attendees: Option<i32>,
    pub max_results: Option<i32>,
    pub order_by: Option<String>,
    pub page_token: Option<String>,
    pub q: Option<String>,
    pub shared_extended_property: Option<String>,
    pub show_deleted: Option<bool>,
    pub show_hidden_invitations: Option<bool>,
    pub single_events: Option<bool>,
    pub sync_token: Option<String>,
    pub time_max: Option<chrono::DateTime<chrono::Local>>,
    pub time_min: Option<chrono::DateTime<chrono::Local>>,
    pub timezone: Option<String>,
    pub updated_min: Option<chrono::DateTime<chrono::Local>>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EventTypes {
    Default,
    FocusTime,
    OutOfOffice,
    WorkingLocation,
}

impl std::fmt::Display for EventTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            EventTypes::Default => "default",
            EventTypes::FocusTime => "focusTime",
            EventTypes::OutOfOffice => "outOfOffice",
            EventTypes::WorkingLocation => "workingLocation",
        };
        write!(f, "{}", s)
    }
}

impl EventClient {
    /// Construct a new EventClient. Requires a Google Calendar Client.
    pub fn new(client: Arc<GCalClient>) -> Self {
        Self(client)
    }

    /// Delete the event.
    pub async fn delete(&self, event: Event) -> ClientResult<()> {
        self.0.delete(None, event).await?;
        Ok(())
    }

    /// Get an event by ID.
    pub async fn get(&self, calendar_id: String, event_id: String) -> ClientResult<Event> {
        let event = Event {
            id: event_id,
            calendar_id,
            ..Default::default()
        };
        Ok(self.0.get(None, event).await?.json().await?)
    }

    /// Import an event. See the Google Calendar documentation for the differences between import
    /// and insert.
    pub async fn import(&self, event: Event) -> ClientResult<Event> {
        Ok(self
            .0
            .post(Some("import".to_string()), event)
            .await?
            .json()
            .await?)
    }

    /// Insert an event. See the Google Calendar documentation for the differences between import
    /// and insert.
    pub async fn insert(&self, mut event: Event) -> ClientResult<Event> {
        if !event.attachments.is_empty() {
            event.add_query("supportsAttachments".to_string(), "true".to_string());
        }
        Ok(self
            .0
            .post(Some(String::new()), event)
            .await?
            .json()
            .await?)
    }

    /// Retrieve all instances for a recurring event.
    pub async fn instances(&self, event: Event) -> ClientResult<Events> {
        Ok(self
            .0
            .get(Some("instances".to_string()), event)
            .await?
            .json()
            .await?)
    }

    /// List events between the start and end times.
    pub async fn list(
        &self,
        calendar_id: String,
        options: Option<EventListOptions>,
    ) -> ClientResult<Vec<Event>> {
        let mut event = Event {
            calendar_id: calendar_id.clone(),
            ..Default::default()
        };
    
        if let Some(opts) = options {
            // Convert the options to query parameters automatically
            let query_params = serde_qs::to_string(&opts)?;
            for (key, value) in serde_qs::from_str::<Vec<(String, String)>>(&query_params)? {
                event.add_query(key, value);
            }
        }
    
        let mut events = self
            .0
            .get(None, event)
            .await
            .expect("Fail here 1")
            .json::<Events>()
            .await
            .expect("Fail 2");
        events.add_calendar(calendar_id);
        Ok(events.items)
    }

    /// Move event to another destination calendar_id.
    pub async fn move_to_calendar(
        &self,
        mut event: Event,
        destination: String,
        send_updates: Option<SendUpdates>,
    ) -> ClientResult<()> {
        event.add_query("destination".to_string(), destination);
        event.add_query(
            "sendUpdates".to_string(),
            send_updates.map_or_else(|| "false".to_string(), |x| x.to_string()),
        );

        self.0.post(Some("move".to_string()), event).await?;
        Ok(())
    }

    /// Add an event with the summary.
    pub async fn add(&self, text: String) -> ClientResult<Event> {
        let mut event = Event::default();
        event.add_query("text".to_string(), text);

        Ok(self
            .0
            .post(Some("quickAdd".to_string()), event)
            .await?
            .json()
            .await?)
    }

    /// Update an event.
    pub async fn update(&self, event: Event) -> ClientResult<Event> {
        Ok(self.0.put(None, event).await?.json().await?)
    }
}
