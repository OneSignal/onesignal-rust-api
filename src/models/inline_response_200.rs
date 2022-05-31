/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse200 {
    #[serde(rename = "id")]
    pub id: String,
    /// Estimated number of subscribers targetted by notification.
    #[serde(rename = "recipients")]
    pub recipients: i32,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<crate::models::Notification200Errors>>,
}

impl InlineResponse200 {
    pub fn new(id: String, recipients: i32) -> InlineResponse200 {
        InlineResponse200 {
            id,
            recipients,
            external_id: None,
            errors: None,
        }
    }
}


