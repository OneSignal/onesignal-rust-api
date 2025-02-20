/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateLiveActivitySuccessResponse {
    #[serde(rename = "notification_id", skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<crate::models::Notification200Errors>>,
}

impl UpdateLiveActivitySuccessResponse {
    pub fn new() -> UpdateLiveActivitySuccessResponse {
        UpdateLiveActivitySuccessResponse {
            notification_id: None,
            errors: None,
        }
    }
}


