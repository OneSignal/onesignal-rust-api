# Rust API client for onesignal-rust-api

A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com

For more information, please visit [https://onesignal.com](https://onesignal.com)

- API version: 5.8.0
- Package version: 5.8.0

## Installation

Add to `Cargo.toml` under `[dependencies]`:

```toml
onesignal-rust-api = "5.8.0"
```

## Configuration

Every SDK requires authentication via API keys. Two key types are available:

- **REST API Key** — required for most endpoints (sending notifications, managing users, etc.). Found in your app's **Settings > Keys & IDs**.
- **Organization API Key** — only required for organization-level endpoints like creating or listing apps. Found in **Organization Settings**.

> **Warning:** Store your API keys in environment variables or a secrets manager. Never commit them to source control.

```rust
use onesignal::apis::configuration::Configuration;

fn create_configuration() -> Configuration {
    let mut config = Configuration::new();
    config.rest_api_key_token = Some("YOUR_REST_API_KEY".to_string());
    config.organization_api_key_token = Some("YOUR_ORGANIZATION_API_KEY".to_string());
    config
}
```

## Send a push notification

```rust
use onesignal::apis::default_api;
use onesignal::models::{Notification, StringMap};

let mut contents = StringMap::new();
contents.en = Some("Hello from OneSignal!".to_string());

let mut headings = StringMap::new();
headings.en = Some("Push Notification".to_string());

let mut notification = Notification::new("YOUR_APP_ID".to_string());
notification.contents = Some(Box::new(contents));
notification.headings = Some(Box::new(headings));
notification.included_segments = Some(vec!["Subscribed Users".to_string()]);

let config = create_configuration();
let response = default_api::create_notification(&config, notification).await;
```

## Send an email

```rust
let mut notification = Notification::new("YOUR_APP_ID".to_string());
notification.email_subject = Some("Important Update".to_string());
notification.email_body = Some("<h1>Hello!</h1><p>This is an HTML email.</p>".to_string());
notification.included_segments = Some(vec!["Subscribed Users".to_string()]);
notification.channel_for_external_user_ids = Some("email".to_string());

let response = default_api::create_notification(&config, notification).await;
```

## Send an SMS

```rust
let mut contents = StringMap::new();
contents.en = Some("Your SMS message content here".to_string());

let mut notification = Notification::new("YOUR_APP_ID".to_string());
notification.contents = Some(Box::new(contents));
notification.included_segments = Some(vec!["Subscribed Users".to_string()]);
notification.channel_for_external_user_ids = Some("sms".to_string());
notification.sms_from = Some("+15551234567".to_string());

let response = default_api::create_notification(&config, notification).await;
```

## Full API reference

The complete list of API endpoints and their parameters is available in the [DefaultApi documentation](https://github.com/OneSignal/onesignal-rust-api/blob/main/docs/DefaultApi.md).

For the underlying REST API, see the [OneSignal API reference](https://documentation.onesignal.com/reference).
