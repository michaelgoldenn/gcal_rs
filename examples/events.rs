//! This example showcases the Google OAuth2 process for requesting access to the Google Calendar features
//! and the user's events.
//!
//! Before running it, you'll need to generate your own Google OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy cargo run --example events
//! ```
//!
//! ...and follow the instructions.
use gcal::*;

#[tokio::main]
async fn main() {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("[ERR] Missing the GOOGLE_CLIENT_ID environment variable.");
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .expect("[ERR] Missing the GOOGLE_CLIENT_SECRET environment variable.");

    let (acc_token, _) = OAuth::naive(client_id, client_secret)
        .await
        .expect("[ERR] Failed to get access key.");

    let (calendar_client, event_client) = GCalClient::new(acc_token).unwrap().clients();

    let list = calendar_client
        .list(true, CalendarAccessRole::Reader)
        .await
        .unwrap();

    let start = chrono::Local::now();
    let end = start + chrono::Duration::days(1);

    let mut event_list = Vec::new();
    for calendar in list {
        if let Ok(e) = event_client.list(calendar.id.clone(), start, end).await {
            event_list.extend(e);
        } else {
            println!("[ERR] Calendar failed: {}", calendar.id);
        }
    }

    println!();
    for event in &event_list {
        println!("Event: {:?}", event.summary);
    }
}
