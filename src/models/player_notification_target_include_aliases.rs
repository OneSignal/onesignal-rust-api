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
pub struct PlayerNotificationTargetIncludeAliases {
    #[serde(rename = "alias_label", skip_serializing_if = "Option::is_none")]
    pub alias_label: Option<Vec<String>>,
}

impl PlayerNotificationTargetIncludeAliases {
    pub fn new() -> PlayerNotificationTargetIncludeAliases {
        PlayerNotificationTargetIncludeAliases {
            alias_label: None,
        }
    }
}


