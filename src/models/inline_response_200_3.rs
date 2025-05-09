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
pub struct InlineResponse2003 {
    #[serde(rename = "in_app_messages", skip_serializing_if = "Option::is_none")]
    pub in_app_messages: Option<Vec<serde_json::Value>>,
}

impl InlineResponse2003 {
    pub fn new() -> InlineResponse2003 {
        InlineResponse2003 {
            in_app_messages: None,
        }
    }
}


