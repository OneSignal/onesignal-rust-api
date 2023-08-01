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
pub struct NotificationHistorySuccessResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "destination_url", skip_serializing_if = "Option::is_none")]
    pub destination_url: Option<String>,
}

impl NotificationHistorySuccessResponse {
    pub fn new() -> NotificationHistorySuccessResponse {
        NotificationHistorySuccessResponse {
            success: None,
            destination_url: None,
        }
    }
}


