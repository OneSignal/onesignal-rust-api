/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.2.1
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationTarget {
    /// The segment names you want to target. Users in these segments will receive a notification. This targeting parameter is only compatible with excluded_segments. Example: [\"Active Users\", \"Inactive Users\"] 
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    /// Segment that will be excluded when sending. Users in these segments will not receive a notification, even if they were included in included_segments. This targeting parameter is only compatible with included_segments. Example: [\"Active Users\", \"Inactive Users\"] 
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Specific playerids to send your notification to. _Does not require API Auth Key. Do not combine with other targeting parameters. Not compatible with any other targeting parameters. Example: [\"1dd608f2-c6a1-11e3-851d-000c2940e62c\"] Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_player_ids", skip_serializing_if = "Option::is_none")]
    pub include_player_ids: Option<Vec<String>>,
    /// Target specific devices by custom user IDs assigned via API. Not compatible with any other targeting parameters Example: [\"custom-id-assigned-by-api\"] REQUIRED: REST API Key Authentication Limit of 2,000 entries per REST API call. Note: If targeting push, email, or sms subscribers with same ids, use with channel_for_external_user_ids to indicate you are sending a push or email or sms. 
    #[serde(rename = "include_external_user_ids", skip_serializing_if = "Option::is_none")]
    pub include_external_user_ids: Option<Vec<String>>,
    /// Recommended for Sending Emails - Target specific email addresses. If an email does not correspond to an existing user, a new user will be created. Example: nick@catfac.ts Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_email_tokens", skip_serializing_if = "Option::is_none")]
    pub include_email_tokens: Option<Vec<String>>,
    /// Recommended for Sending SMS - Target specific phone numbers. The phone number should be in the E.164 format. Phone number should be an existing subscriber on OneSignal. Refer our docs to learn how to add phone numbers to OneSignal. Example phone number: +1999999999 Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_phone_numbers", skip_serializing_if = "Option::is_none")]
    pub include_phone_numbers: Option<Vec<String>>,
    /// Not Recommended: Please consider using include_player_ids or include_external_user_ids instead. Target using iOS device tokens. Warning: Only works with Production tokens. All non-alphanumeric characters must be removed from each token. If a token does not correspond to an existing user, a new user will be created. Example: ce777617da7f548fe7a9ab6febb56cf39fba6d38203... Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_ios_tokens", skip_serializing_if = "Option::is_none")]
    pub include_ios_tokens: Option<Vec<String>>,
    /// Not Recommended: Please consider using include_player_ids or include_external_user_ids instead. Target using Windows URIs. If a token does not correspond to an existing user, a new user will be created. Example: http://s.notify.live.net/u/1/bn1/HmQAAACPaLDr-... Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_wp_wns_uris", skip_serializing_if = "Option::is_none")]
    pub include_wp_wns_uris: Option<Vec<String>>,
    /// Not Recommended: Please consider using include_player_ids or include_external_user_ids instead. Target using Amazon ADM registration IDs. If a token does not correspond to an existing user, a new user will be created. Example: amzn1.adm-registration.v1.XpvSSUk0Rc3hTVVV... Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_amazon_reg_ids", skip_serializing_if = "Option::is_none")]
    pub include_amazon_reg_ids: Option<Vec<String>>,
    /// Not Recommended: Please consider using include_player_ids or include_external_user_ids instead. Target using Chrome App registration IDs. If a token does not correspond to an existing user, a new user will be created. Example: APA91bEeiUeSukAAUdnw3O2RB45FWlSpgJ7Ji_... Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_chrome_reg_ids", skip_serializing_if = "Option::is_none")]
    pub include_chrome_reg_ids: Option<Vec<String>>,
    /// Not Recommended: Please consider using include_player_ids or include_external_user_ids instead. Target using Chrome Web Push registration IDs. If a token does not correspond to an existing user, a new user will be created. Example: APA91bEeiUeSukAAUdnw3O2RB45FWlSpgJ7Ji_... Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_chrome_web_reg_ids", skip_serializing_if = "Option::is_none")]
    pub include_chrome_web_reg_ids: Option<Vec<String>>,
    /// Not Recommended: Please consider using include_player_ids or include_external_user_ids instead. Target using Android device registration IDs. If a token does not correspond to an existing user, a new user will be created. Example: APA91bEeiUeSukAAUdnw3O2RB45FWlSpgJ7Ji_... Limit of 2,000 entries per REST API call 
    #[serde(rename = "include_android_reg_ids", skip_serializing_if = "Option::is_none")]
    pub include_android_reg_ids: Option<Vec<String>>,
    #[serde(rename = "include_aliases", skip_serializing_if = "Option::is_none")]
    pub include_aliases: Option<Box<crate::models::PlayerNotificationTargetIncludeAliases>>,
    #[serde(rename = "target_channel", skip_serializing_if = "Option::is_none")]
    pub target_channel: Option<TargetChannelType>,
}

impl NotificationTarget {
    pub fn new() -> NotificationTarget {
        NotificationTarget {
            included_segments: None,
            excluded_segments: None,
            include_player_ids: None,
            include_external_user_ids: None,
            include_email_tokens: None,
            include_phone_numbers: None,
            include_ios_tokens: None,
            include_wp_wns_uris: None,
            include_amazon_reg_ids: None,
            include_chrome_reg_ids: None,
            include_chrome_web_reg_ids: None,
            include_android_reg_ids: None,
            include_aliases: None,
            target_channel: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetChannelType {
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
}

impl Default for TargetChannelType {
    fn default() -> TargetChannelType {
        Self::Push
    }
}

