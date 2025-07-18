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
pub struct PropertiesDeltas {
    #[serde(rename = "session_time", skip_serializing_if = "Option::is_none")]
    pub session_time: Option<i32>,
    #[serde(rename = "session_count", skip_serializing_if = "Option::is_none")]
    pub session_count: Option<i32>,
    #[serde(rename = "purchases", skip_serializing_if = "Option::is_none")]
    pub purchases: Option<Vec<crate::models::Purchase>>,
}

impl PropertiesDeltas {
    pub fn new() -> PropertiesDeltas {
        PropertiesDeltas {
            session_time: None,
            session_count: None,
            purchases: None,
        }
    }
}


