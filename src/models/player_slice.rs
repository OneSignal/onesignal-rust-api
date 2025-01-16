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
pub struct PlayerSlice {
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<crate::models::Player>>,
}

impl PlayerSlice {
    pub fn new() -> PlayerSlice {
        PlayerSlice {
            total_count: None,
            offset: None,
            limit: None,
            players: None,
        }
    }
}


