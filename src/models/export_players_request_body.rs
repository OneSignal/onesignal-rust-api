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
pub struct ExportPlayersRequestBody {
    /// Additional fields that you wish to include. Currently supports location, country, rooted, notification_types, ip, external_user_id, web_auth, and web_p256.
    #[serde(rename = "extra_fields", skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<String>>,
    /// Export all devices with a last_active timestamp greater than this time.  Unixtime in seconds.
    #[serde(rename = "last_active_since", skip_serializing_if = "Option::is_none")]
    pub last_active_since: Option<String>,
    /// Export all devices belonging to the segment.
    #[serde(rename = "segment_name", skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

impl ExportPlayersRequestBody {
    pub fn new() -> ExportPlayersRequestBody {
        ExportPlayersRequestBody {
            extra_fields: None,
            last_active_since: None,
            segment_name: None,
        }
    }
}


