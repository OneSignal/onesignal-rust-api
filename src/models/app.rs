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
pub struct App {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of your app, as displayed on your apps list on the dashboard.  This can be renamed.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<i32>,
    #[serde(rename = "messageable_players", skip_serializing_if = "Option::is_none")]
    pub messageable_players: Option<i32>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Android: Your Google Project number.  Also known as Sender ID.
    #[serde(rename = "android_gcm_sender_id", skip_serializing_if = "Option::is_none")]
    pub android_gcm_sender_id: Option<String>,
    /// Android: Your Google Push Messaging Auth Key
    #[serde(rename = "gcm_key", skip_serializing_if = "Option::is_none")]
    pub gcm_key: Option<String>,
    /// Chrome (All Browsers except Safari) (Recommended): The URL to your website.  This field is required if you wish to enable web push and specify other web push parameters.
    #[serde(rename = "chrome_web_origin", skip_serializing_if = "Option::is_none")]
    pub chrome_web_origin: Option<String>,
    /// Not for web push.  Your Google Push Messaging Auth Key if you use Chrome Apps / Extensions.
    #[serde(rename = "chrome_key", skip_serializing_if = "Option::is_none")]
    pub chrome_key: Option<String>,
    /// Chrome (All Browsers except Safari): Your default notification icon. Should be 256x256 pixels, min 80x80.
    #[serde(rename = "chrome_web_default_notification_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_web_default_notification_icon: Option<String>,
    /// Chrome (All Browsers except Safari): A subdomain of your choice in order to support Web Push on non-HTTPS websites. This field must be set in order for the chrome_web_gcm_sender_id property to be processed.
    #[serde(rename = "chrome_web_sub_domain", skip_serializing_if = "Option::is_none")]
    pub chrome_web_sub_domain: Option<String>,
    /// iOS: Either sandbox or production
    #[serde(rename = "apns_env", skip_serializing_if = "Option::is_none")]
    pub apns_env: Option<ApnsEnvType>,
    /// iOS: Your apple push notification p12 certificate file, converted to a string and Base64 encoded.
    #[serde(rename = "apns_p12", skip_serializing_if = "Option::is_none")]
    pub apns_p12: Option<String>,
    /// iOS: Required if using p12 certificate.  Password for the apns_p12 file.
    #[serde(rename = "apns_p12_password", skip_serializing_if = "Option::is_none")]
    pub apns_p12_password: Option<String>,
    #[serde(rename = "apns_certificates", skip_serializing_if = "Option::is_none")]
    pub apns_certificates: Option<String>,
    #[serde(rename = "safari_apns_certificates", skip_serializing_if = "Option::is_none")]
    pub safari_apns_certificates: Option<String>,
    /// Safari: Your apple push notification p12 certificate file for Safari Push Notifications, converted to a string and Base64 encoded.
    #[serde(rename = "safari_apns_p12", skip_serializing_if = "Option::is_none")]
    pub safari_apns_p12: Option<String>,
    /// Safari: Password for safari_apns_p12 file
    #[serde(rename = "safari_apns_p12_password", skip_serializing_if = "Option::is_none")]
    pub safari_apns_p12_password: Option<String>,
    /// iOS: Required if using p8. Unique identifier for the p8 authentication key.
    #[serde(rename = "apns_key_id", skip_serializing_if = "Option::is_none")]
    pub apns_key_id: Option<String>,
    /// iOS: Required if using p8. Team ID generated by Apple for your developer account.
    #[serde(rename = "apns_team_id", skip_serializing_if = "Option::is_none")]
    pub apns_team_id: Option<String>,
    /// iOS: Required if using p8. Bundle ID for your app in the Apple ecosystem.
    #[serde(rename = "apns_bundle_id", skip_serializing_if = "Option::is_none")]
    pub apns_bundle_id: Option<String>,
    /// iOS: Required if using p8. Base64 encoded p8 key
    #[serde(rename = "apns_p8", skip_serializing_if = "Option::is_none")]
    pub apns_p8: Option<String>,
    /// Safari (Recommended): The hostname to your website including http(s)://
    #[serde(rename = "safari_site_origin", skip_serializing_if = "Option::is_none")]
    pub safari_site_origin: Option<String>,
    #[serde(rename = "safari_push_id", skip_serializing_if = "Option::is_none")]
    pub safari_push_id: Option<String>,
    #[serde(rename = "safari_icon_16_16", skip_serializing_if = "Option::is_none")]
    pub safari_icon_16_16: Option<String>,
    #[serde(rename = "safari_icon_32_32", skip_serializing_if = "Option::is_none")]
    pub safari_icon_32_32: Option<String>,
    #[serde(rename = "safari_icon_64_64", skip_serializing_if = "Option::is_none")]
    pub safari_icon_64_64: Option<String>,
    #[serde(rename = "safari_icon_128_128", skip_serializing_if = "Option::is_none")]
    pub safari_icon_128_128: Option<String>,
    /// Safari: A url for a 256x256 png notification icon. This is the only Safari icon URL you need to provide.
    #[serde(rename = "safari_icon_256_256", skip_serializing_if = "Option::is_none")]
    pub safari_icon_256_256: Option<String>,
    /// All Browsers (Recommended): The Site Name. Requires both chrome_web_origin and safari_site_origin to be set to add or update it.
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(rename = "basic_auth_key", skip_serializing_if = "Option::is_none")]
    pub basic_auth_key: Option<String>,
    /// The Id of the Organization you would like to add this app to.
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// iOS: Notification data (additional data) values will be added to the root of the apns payload when sent to the device.  Ignore if you're not using any other plugins, or not using OneSignal SDK methods to read the payload.
    #[serde(rename = "additional_data_is_root_payload", skip_serializing_if = "Option::is_none")]
    pub additional_data_is_root_payload: Option<bool>,
}

impl App {
    pub fn new() -> App {
        App {
            id: None,
            name: None,
            players: None,
            messageable_players: None,
            updated_at: None,
            created_at: None,
            android_gcm_sender_id: None,
            gcm_key: None,
            chrome_web_origin: None,
            chrome_key: None,
            chrome_web_default_notification_icon: None,
            chrome_web_sub_domain: None,
            apns_env: None,
            apns_p12: None,
            apns_p12_password: None,
            apns_certificates: None,
            safari_apns_certificates: None,
            safari_apns_p12: None,
            safari_apns_p12_password: None,
            apns_key_id: None,
            apns_team_id: None,
            apns_bundle_id: None,
            apns_p8: None,
            safari_site_origin: None,
            safari_push_id: None,
            safari_icon_16_16: None,
            safari_icon_32_32: None,
            safari_icon_64_64: None,
            safari_icon_128_128: None,
            safari_icon_256_256: None,
            site_name: None,
            basic_auth_key: None,
            organization_id: None,
            additional_data_is_root_payload: None,
        }
    }
}

/// iOS: Either sandbox or production
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApnsEnvType {
    #[serde(rename = "sandbox")]
    Sandbox,
    #[serde(rename = "production")]
    Production,
}

impl Default for ApnsEnvType {
    fn default() -> ApnsEnvType {
        Self::Sandbox
    }
}

