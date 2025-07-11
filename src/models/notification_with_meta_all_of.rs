/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 5.2.0
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationWithMetaAllOf {
    /// Number of notifications that have not been sent out yet. This can mean either our system is still processing the notification or you have delayed options set.
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i32>,
    /// Number of notifications that were successfully delivered.
    #[serde(rename = "successful", skip_serializing_if = "Option::is_none")]
    pub successful: Option<i32>,
    /// Number of notifications that could not be delivered due to those devices being unsubscribed.
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// Number of notifications that could not be delivered due to an error. You can find more information by viewing the notification in the dashboard.
    #[serde(rename = "errored", skip_serializing_if = "Option::is_none")]
    pub errored: Option<i32>,
    /// Number of users who have clicked / tapped on your notification.
    #[serde(rename = "converted", skip_serializing_if = "Option::is_none")]
    pub converted: Option<i32>,
    /// Unix timestamp indicating when the notification was created.
    #[serde(rename = "queued_at", skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<i64>,
    /// Unix timestamp indicating when notification delivery should begin.
    #[serde(rename = "send_after", skip_serializing_if = "Option::is_none")]
    pub send_after: Option<i64>,
    /// Unix timestamp indicating when notification delivery completed. The delivery duration from start to finish can be calculated with completed_at - send_after.
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    #[serde(rename = "platform_delivery_stats", skip_serializing_if = "Option::is_none")]
    pub platform_delivery_stats: Option<Box<crate::models::PlatformDeliveryData>>,
    /// Confirmed Deliveries number of devices that received the push notification. Paid Feature Only. Free accounts will see 0.
    #[serde(rename = "received", skip_serializing_if = "Option::is_none")]
    pub received: Option<i32>,
    /// number of push notifications sent per minute. Paid Feature Only. If throttling is not enabled for the app or the notification, and for free accounts, null is returned. Refer to Throttling for more details.
    #[serde(rename = "throttle_rate_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_rate_per_minute: Option<i32>,
}

impl NotificationWithMetaAllOf {
    pub fn new() -> NotificationWithMetaAllOf {
        NotificationWithMetaAllOf {
            remaining: None,
            successful: None,
            failed: None,
            errored: None,
            converted: None,
            queued_at: None,
            send_after: None,
            completed_at: None,
            platform_delivery_stats: None,
            received: None,
            throttle_rate_per_minute: None,
        }
    }
}


