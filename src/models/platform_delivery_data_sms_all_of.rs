/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlatformDeliveryDataSmsAllOf {
    /// Number of messages reported as delivered successfully by the SMS service provider.
    #[serde(rename = "provider_successful", skip_serializing_if = "Option::is_none")]
    pub provider_successful: Option<i32>,
    /// Number of recipients who didn't receive your message as reported by the SMS service provider.
    #[serde(rename = "provider_failed", skip_serializing_if = "Option::is_none")]
    pub provider_failed: Option<i32>,
    /// Number of errors reported by the SMS service provider.
    #[serde(rename = "provider_errored", skip_serializing_if = "Option::is_none")]
    pub provider_errored: Option<i32>,
}

impl PlatformDeliveryDataSmsAllOf {
    pub fn new() -> PlatformDeliveryDataSmsAllOf {
        PlatformDeliveryDataSmsAllOf {
            provider_successful: None,
            provider_failed: None,
            provider_errored: None,
        }
    }
}


