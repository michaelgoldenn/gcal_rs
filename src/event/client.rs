use std::{sync::Arc, default::Default};

use super::{ClientResult, Event, Events, GCalClient, SendUpdates, EventListOptions};

/// EventClient is the method of managing events from a specific calendar. Requires a Google
/// Calendar client.
#[derive(Debug, Clone)]
pub struct EventClient(Arc<GCalClient>);

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
