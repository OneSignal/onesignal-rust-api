/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.2.2
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeliveryData {
    /// Number of messages delivered to push servers, mobile carriers, or email service providers.
    #[serde(rename = "successful", skip_serializing_if = "Option::is_none")]
    pub successful: Option<i32>,
    /// Number of messages sent to unsubscribed devices.
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// Number of errors reported.
    #[serde(rename = "errored", skip_serializing_if = "Option::is_none")]
    pub errored: Option<i32>,
    /// Number of messages that were clicked.
    #[serde(rename = "converted", skip_serializing_if = "Option::is_none")]
    pub converted: Option<i32>,
    /// Number of devices that received the message.
    #[serde(rename = "received", skip_serializing_if = "Option::is_none")]
    pub received: Option<i32>,
}

impl DeliveryData {
    pub fn new() -> DeliveryData {
        DeliveryData {
            successful: None,
            failed: None,
            errored: None,
            converted: None,
            received: None,
        }
    }
}


