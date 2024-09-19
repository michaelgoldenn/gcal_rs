//! This example showcases the Google OAuth2 process for requesting access to the Google Calendar features
//! and the user's profile.
//!
//! Before running it, you'll need to generate your own Google OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy cargo run --example calendars
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

    let calendar_client = GCalClient::new(acc_token).unwrap().calendar_client();
    let list = calendar_client
        .list(true, CalendarAccessRole::Reader)
        .await
        .unwrap();

    println!();
    for cal in &list {
        eprintln!("Calendar: {} {}", cal.id, cal.summary);
    }
}
