//! Helpers for common OneSignal API usage patterns.

use std::time::Duration;

use crate::apis::configuration;
use crate::apis::default_api::CreateNotificationError;
use crate::apis::{Error, ResponseContent};
use crate::models;

const RETRYABLE_STATUSES: [u16; 2] = [429, 503];
const MIN_BASE_DELAY: Duration = Duration::from_secs(1);
const MAX_BASE_DELAY: Duration = Duration::from_secs(60);

/// Options for [`create_notification_with_retry`].
#[derive(Debug, Clone)]
pub struct CreateNotificationWithRetryOptions {
    /// Retries after the initial attempt. Default 3.
    pub max_retries: u32,
    /// Backoff base used when the response carries no `Retry-After` header.
    /// Clamped to [1s, 60s]. Default 1s.
    pub base_delay: Duration,
}

impl Default for CreateNotificationWithRetryOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_secs(1),
        }
    }
}

/// Result of [`create_notification_with_retry`]: the create response plus
/// whether the server replayed a previously completed request
/// (`Idempotent-Replayed` response header).
#[derive(Debug, Clone)]
pub struct CreateNotificationWithRetryResult {
    pub response: models::CreateNotificationSuccessResponse,
    pub was_replayed: bool,
}

enum AttemptOutcome {
    Fatal(Error<CreateNotificationError>),
    Retryable(Error<CreateNotificationError>, Option<Duration>),
}

/// Create a notification with safe, idempotent retries.
///
/// Ensures `notification.idempotency_key` is set (generating a UUIDv4 when
/// absent) so the server can deduplicate, then posts to `/notifications`.
/// Transient failures (HTTP 429, HTTP 503, or connect/timeout errors) are
/// retried with the SAME idempotency key, honoring the `Retry-After` response
/// header when present and falling back to exponential backoff
/// (`base_delay * 2^attempt`) otherwise. Other errors are returned
/// immediately. An existing idempotency key is respected, never overwritten.
///
/// NOTE: the request logic below mirrors `apis::default_api::create_notification`
/// (which does not expose response headers); keep the two in sync when the
/// generated operation changes.
pub async fn create_notification_with_retry(
    configuration: &configuration::Configuration,
    notification: &mut models::Notification,
    options: Option<CreateNotificationWithRetryOptions>,
) -> Result<CreateNotificationWithRetryResult, Error<CreateNotificationError>> {
    let opts = options.unwrap_or_default();
    // Clamp the backoff base so a stray value can neither hammer the API (too
    // small) nor stall the caller for an unbounded stretch (too large).
    let base_delay = opts.base_delay.clamp(MIN_BASE_DELAY, MAX_BASE_DELAY);

    if notification
        .idempotency_key
        .as_deref()
        .map_or(true, |key| key.is_empty())
    {
        notification.idempotency_key = Some(uuid::Uuid::new_v4().to_string());
    }

    let mut attempt: u32 = 0;
    loop {
        match send_once(configuration, notification).await {
            Ok(result) => return Ok(result),
            Err(AttemptOutcome::Fatal(error)) => return Err(error),
            Err(AttemptOutcome::Retryable(error, retry_after)) => {
                if attempt >= opts.max_retries {
                    return Err(error);
                }
                let delay =
                    retry_after.unwrap_or_else(|| base_delay * 2u32.saturating_pow(attempt));
                if !delay.is_zero() {
                    tokio::time::sleep(delay).await;
                }
                attempt += 1;
            }
        }
    }
}

async fn send_once(
    configuration: &configuration::Configuration,
    notification: &models::Notification,
) -> Result<CreateNotificationWithRetryResult, AttemptOutcome> {
    let client = &configuration.client;

    let uri_str = format!("{}/notifications", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    // Adds a telemetry header
    req_builder = req_builder.header(
        "OS-Usage-Data",
        concat!(
            "kind=sdk, sdk-name=onesignal-rust, version=",
            env!("CARGO_PKG_VERSION")
        ),
    );

    if let Some(ref token) = configuration.rest_api_key_token {
        req_builder = req_builder.header("Authorization", format!("Key {}", token.to_owned()));
    }
    req_builder = req_builder.json(notification);

    let req = req_builder
        .build()
        .map_err(|e| AttemptOutcome::Fatal(Error::Reqwest(e)))?;
    let resp = match client.execute(req).await {
        Ok(resp) => resp,
        Err(e) => {
            return Err(if e.is_timeout() || e.is_connect() {
                AttemptOutcome::Retryable(Error::Reqwest(e), None)
            } else {
                AttemptOutcome::Fatal(Error::Reqwest(e))
            });
        }
    };

    let status = resp.status();
    let was_replayed = header_value(&resp, "idempotent-replayed")
        .map_or(false, |value| value.trim().eq_ignore_ascii_case("true"));
    let retry_after = header_value(&resp, "retry-after")
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(Duration::from_secs);

    let content = resp
        .text()
        .await
        .map_err(|e| AttemptOutcome::Fatal(Error::Reqwest(e)))?;

    if !status.is_client_error() && !status.is_server_error() {
        let response = serde_json::from_str(&content)
            .map_err(|e| AttemptOutcome::Fatal(Error::Serde(e)))?;
        Ok(CreateNotificationWithRetryResult {
            response,
            was_replayed,
        })
    } else {
        let entity: Option<CreateNotificationError> = serde_json::from_str(&content).ok();
        let error = Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        });
        Err(if RETRYABLE_STATUSES.contains(&status.as_u16()) {
            AttemptOutcome::Retryable(error, retry_after)
        } else {
            AttemptOutcome::Fatal(error)
        })
    }
}

fn header_value(resp: &reqwest::Response, name: &str) -> Option<String> {
    resp.headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_owned())
}
