# AGENTS.md — OneSignal Rust SDK

Integration guide for AI agents and LLMs using `onesignal-rust-api`, the OneSignal server SDK for Rust. Human-oriented docs are in the [README](./README.md).

## What this SDK does

Server-side access to the OneSignal REST API: send push / email / SMS, manage users, subscriptions, segments, templates and live activities, and administer apps & API keys.

- Crate: `onesignal-rust-api` (import path `onesignal_rust_api`)
- Repository: https://github.com/OneSignal/onesignal-rust-api

## Install

```toml
[dependencies]
onesignal-rust-api = "5"
```

## Authentication — two key types

OneSignal uses two bearer credentials; each endpoint requires a specific one. Set both fields on the `Configuration`:

- **REST API key** (`rest_api_key_token`) — used by almost every endpoint (notifications, users, subscriptions, segments, templates, live activities, custom events). Found in **App Settings → Keys & IDs**.
- **Organization API key** (`organization_api_key_token`) — required *only* for organization-level endpoints: app management (list / create / get / update apps), API-key management (view / create / update / rotate / delete API keys), and copying a template to another app. Found in **Organization Settings**.

The SDK sends the correct credential per endpoint. Never hard-code keys — read them from environment variables or a secrets manager.

```rust
use onesignal_rust_api::apis::configuration::Configuration;

let mut configuration = Configuration::new();
configuration.rest_api_key_token = Some(std::env::var("ONESIGNAL_REST_API_KEY").unwrap());
configuration.organization_api_key_token = Some(std::env::var("ONESIGNAL_ORGANIZATION_API_KEY").unwrap());
```

## Calling convention

Calls are free functions in the `default_api` module that take `&Configuration` followed by the model — **positional arguments**, no request/options wrapper.

```rust
use onesignal_rust_api::apis::default_api;
use onesignal_rust_api::models::{Notification, LanguageStringMap};
use onesignal_rust_api::models::notification::TargetChannelType;

let mut notification = Notification::new("YOUR_APP_ID".to_string());
notification.contents = Some(Box::new(LanguageStringMap {
    en: Some("Hello from OneSignal!".to_string()),
    ..Default::default()
}));
let mut aliases = std::collections::HashMap::new();
aliases.insert("external_id".to_string(), vec!["YOUR_USER_EXTERNAL_ID".to_string()]);
notification.include_aliases = Some(aliases);
notification.target_channel = Some(TargetChannelType::Push);

let response = default_api::create_notification(&configuration, notification).await?;
```

## Idempotent sends & retries

Set `idempotency_key` (a UUID) so a create-notification request can be safely retried — the server returns the original result instead of sending twice. The `create_notification_with_retry` helper handles this for you: it generates an `idempotency_key` when absent, retries `429` / `503` / transport errors with the **same** key (honoring `Retry-After`), and reports via `was_replayed` whether the server answered from a previously completed request.

```rust
use onesignal_rust_api::helpers;

let result = helpers::create_notification_with_retry(&configuration, &mut notification, None).await?;
println!("id: {:?} replayed: {}", result.response.id, result.was_replayed);
```

> The notification-level `external_id` field is the **deprecated** idempotency mechanism — prefer `idempotency_key`. Don't confuse it with the `external_id` **alias label** (under `include_aliases`) used to target users.

## Full API reference

- [DefaultApi.md](https://github.com/OneSignal/onesignal-rust-api/blob/main/docs/DefaultApi.md) — every endpoint, parameter, and model, with runnable examples.
- [OneSignal REST API reference](https://documentation.onesignal.com/reference)
