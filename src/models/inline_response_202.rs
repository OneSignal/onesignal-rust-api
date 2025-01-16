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
pub struct InlineResponse202 {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<crate::models::PropertiesObject>>,
}

impl InlineResponse202 {
    pub fn new() -> InlineResponse202 {
        InlineResponse202 {
            properties: None,
        }
    }
}


