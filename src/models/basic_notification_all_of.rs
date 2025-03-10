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
pub struct BasicNotificationAllOf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    /// Required for SMS Messages. An identifier for tracking message within the OneSignal dashboard or export analytics. Not shown to end user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<AggregationType>,
    /// Indicates whether to send to all devices registered under your app's Apple iOS platform.
    #[serde(rename = "isIos", skip_serializing_if = "Option::is_none")]
    pub is_ios: Option<bool>,
    /// Indicates whether to send to all devices registered under your app's Google Android platform.
    #[serde(rename = "isAndroid", skip_serializing_if = "Option::is_none")]
    pub is_android: Option<bool>,
    /// Indicates whether to send to all devices registered under your app's Huawei Android platform.
    #[serde(rename = "isHuawei", skip_serializing_if = "Option::is_none")]
    pub is_huawei: Option<bool>,
    /// Indicates whether to send to all subscribed web browser users, including Chrome, Firefox, and Safari. You may use this instead as a combined flag instead of separately enabling isChromeWeb, isFirefox, and isSafari, though the three options are equivalent to this one. 
    #[serde(rename = "isAnyWeb", skip_serializing_if = "Option::is_none")]
    pub is_any_web: Option<bool>,
    /// Indicates whether to send to all Google Chrome, Chrome on Android, and Mozilla Firefox users registered under your Chrome & Firefox web push platform.
    #[serde(rename = "isChromeWeb", skip_serializing_if = "Option::is_none")]
    pub is_chrome_web: Option<bool>,
    /// Indicates whether to send to all Mozilla Firefox desktop users registered under your Firefox web push platform.
    #[serde(rename = "isFirefox", skip_serializing_if = "Option::is_none")]
    pub is_firefox: Option<bool>,
    /// Does not support iOS Safari. Indicates whether to send to all Apple's Safari desktop users registered under your Safari web push platform. Read more iOS Safari
    #[serde(rename = "isSafari", skip_serializing_if = "Option::is_none")]
    pub is_safari: Option<bool>,
    /// Indicates whether to send to all devices registered under your app's Windows platform.
    #[serde(rename = "isWP_WNS", skip_serializing_if = "Option::is_none")]
    pub is_wp_wns: Option<bool>,
    /// Indicates whether to send to all devices registered under your app's Amazon Fire platform.
    #[serde(rename = "isAdm", skip_serializing_if = "Option::is_none")]
    pub is_adm: Option<bool>,
    /// This flag is not used for web push Please see isChromeWeb for sending to web push users. This flag only applies to Google Chrome Apps & Extensions. Indicates whether to send to all devices registered under your app's Google Chrome Apps & Extension platform. 
    #[serde(rename = "isChrome", skip_serializing_if = "Option::is_none")]
    pub is_chrome: Option<bool>,
    /// Indicates if the message type when targeting with include_external_user_ids for cases where an email, sms, and/or push subscribers have the same external user id. Example: Use the string \"push\" to indicate you are sending a push notification or the string \"email\"for sending emails or \"sms\"for sending SMS. 
    #[serde(rename = "channel_for_external_user_ids", skip_serializing_if = "Option::is_none")]
    pub channel_for_external_user_ids: Option<String>,
    /// Required: Your OneSignal Application ID, which can be found in Keys & IDs. It is a UUID and looks similar to 8250eaf6-1a58-489e-b136-7c74a864b434. 
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// [DEPRECATED] Correlation and idempotency key. A request received with this parameter will first look for another notification with the same external_id. If one exists, a notification will not be sent, and result of the previous operation will instead be returned. Therefore, if you plan on using this feature, it's important to use a good source of randomness to generate the UUID passed here. This key is only idempotent for 30 days. After 30 days, the notification could be removed from our system and a notification with the same external_id will be sent again.   See Idempotent Notification Requests for more details writeOnly: true 
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Correlation and idempotency key. A request received with this parameter will first look for another notification with the same idempotency key. If one exists, a notification will not be sent, and result of the previous operation will instead be returned. Therefore, if you plan on using this feature, it's important to use a good source of randomness to generate the UUID passed here. This key is only idempotent for 30 days. After 30 days, the notification could be removed from our system and a notification with the same idempotency key will be sent again.   See Idempotent Notification Requests for more details writeOnly: true 
    #[serde(rename = "idempotency_key", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Box<crate::models::StringMap>>,
    #[serde(rename = "headings", skip_serializing_if = "Option::is_none")]
    pub headings: Option<Box<crate::models::StringMap>>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Box<crate::models::StringMap>>,
    /// Channel: Push Notifications Platform: Huawei A custom map of data that is passed back to your app. Same as using Additional Data within the dashboard. Can use up to 2048 bytes of data. Example: {\"abc\": 123, \"foo\": \"bar\", \"event_performed\": true, \"amount\": 12.1} 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Channel: Push Notifications Platform: Huawei Use \"data\" or \"message\" depending on the type of notification you are sending. More details in Data & Background Notifications. 
    #[serde(rename = "huawei_msg_type", skip_serializing_if = "Option::is_none")]
    pub huawei_msg_type: Option<String>,
    /// Channel: Push Notifications Platform: All The URL to open in the browser when a user clicks on the notification. Note: iOS needs https or updated NSAppTransportSecurity in plist This field supports inline substitutions. Omit if including web_url or app_url Example: https://onesignal.com 
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Channel: Push Notifications Platform: All Browsers Same as url but only sent to web push platforms. Including Chrome, Firefox, Safari, Opera, etc. Example: https://onesignal.com 
    #[serde(rename = "web_url", skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    /// Channel: Push Notifications Platform: All Browsers Same as url but only sent to web push platforms. Including iOS, Android, macOS, Windows, ChromeApps, etc. Example: https://onesignal.com 
    #[serde(rename = "app_url", skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    /// Channel: Push Notifications Platform: iOS 10+ Adds media attachments to notifications. Set as JSON object, key as a media id of your choice and the value as a valid local filename or URL. User must press and hold on the notification to view. Do not set mutable_content to download attachments. The OneSignal SDK does this automatically Example: {\"id1\": \"https://domain.com/image.jpg\"} 
    #[serde(rename = "ios_attachments", skip_serializing_if = "Option::is_none")]
    pub ios_attachments: Option<serde_json::Value>,
    /// Channel: Push Notifications Platform: All Use a template you setup on our dashboard. The template_id is the UUID found in the URL when viewing a template on our dashboard. Example: be4a8044-bbd6-11e4-a581-000c2940e62c 
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Channel: Push Notifications Platform: iOS Sending true wakes your app from background to run custom native code (Apple interprets this as content-available=1). Note: Not applicable if the app is in the \"force-quit\" state (i.e app was swiped away). Omit the contents field to prevent displaying a visible notification. 
    #[serde(rename = "content_available", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<bool>,
    /// Channel: Push Notifications Platform: iOS 10+ Always defaults to true and cannot be turned off. Allows tracking of notification receives and changing of the notification content in your app before it is displayed. Triggers didReceive(_:withContentHandler:) on your UNNotificationServiceExtension. 
    #[serde(rename = "mutable_content", skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<bool>,
    /// Channel: Push Notifications Platform: iOS 13+ Use to target a specific experience in your App Clip, or to target your notification to a specific window in a multi-scene App. 
    #[serde(rename = "target_content_identifier", skip_serializing_if = "Option::is_none")]
    pub target_content_identifier: Option<String>,
    /// Channel: Push Notifications Platform: Android Picture to display in the expanded view. Can be a drawable resource name or a URL. 
    #[serde(rename = "big_picture", skip_serializing_if = "Option::is_none")]
    pub big_picture: Option<String>,
    /// Channel: Push Notifications Platform: Huawei Picture to display in the expanded view. Can be a drawable resource name or a URL. 
    #[serde(rename = "huawei_big_picture", skip_serializing_if = "Option::is_none")]
    pub huawei_big_picture: Option<String>,
    /// Channel: Push Notifications Platform: Amazon Picture to display in the expanded view. Can be a drawable resource name or a URL. 
    #[serde(rename = "adm_big_picture", skip_serializing_if = "Option::is_none")]
    pub adm_big_picture: Option<String>,
    /// Channel: Push Notifications Platform: ChromeApp Large picture to display below the notification text. Must be a local URL. 
    #[serde(rename = "chrome_big_picture", skip_serializing_if = "Option::is_none")]
    pub chrome_big_picture: Option<String>,
    /// Channel: Push Notifications Platform: Chrome 56+ Sets the web push notification's large image to be shown below the notification's title and text. Please see Web Push Notification Icons. 
    #[serde(rename = "chrome_web_image", skip_serializing_if = "Option::is_none")]
    pub chrome_web_image: Option<String>,
    /// Channel: Push Notifications Platform: iOS 8.0+, Android 4.1+, and derivatives like Amazon Buttons to add to the notification. Icon only works for Android. Buttons show in reverse order of array position i.e. Last item in array shows as first button on device. Example: [{\"id\": \"id2\", \"text\": \"second button\", \"icon\": \"ic_menu_share\"}, {\"id\": \"id1\", \"text\": \"first button\", \"icon\": \"ic_menu_send\"}] 
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<crate::models::Button>>,
    /// Channel: Push Notifications Platform: Chrome 48+ Add action buttons to the notification. The id field is required. Example: [{\"id\": \"like-button\", \"text\": \"Like\", \"icon\": \"http://i.imgur.com/N8SN8ZS.png\", \"url\": \"https://yoursite.com\"}, {\"id\": \"read-more-button\", \"text\": \"Read more\", \"icon\": \"http://i.imgur.com/MIxJp1L.png\", \"url\": \"https://yoursite.com\"}] 
    #[serde(rename = "web_buttons", skip_serializing_if = "Option::is_none")]
    pub web_buttons: Option<Vec<crate::models::Button>>,
    /// Channel: Push Notifications Platform: iOS Category APS payload, use with registerUserNotificationSettings:categories in your Objective-C / Swift code. Example: calendar category which contains actions like accept and decline iOS 10+ This will trigger your UNNotificationContentExtension whose ID matches this category. 
    #[serde(rename = "ios_category", skip_serializing_if = "Option::is_none")]
    pub ios_category: Option<String>,
    /// Channel: Push Notifications Platform: Android The Android Oreo Notification Category to send the notification under. See the Category documentation on creating one and getting it's id. 
    #[serde(rename = "android_channel_id", skip_serializing_if = "Option::is_none")]
    pub android_channel_id: Option<String>,
    /// Channel: Push Notifications Platform: Huawei The Android Oreo Notification Category to send the notification under. See the Category documentation on creating one and getting it's id. 
    #[serde(rename = "huawei_channel_id", skip_serializing_if = "Option::is_none")]
    pub huawei_channel_id: Option<String>,
    /// Channel: Push Notifications Platform: Android Use this if you have client side Android Oreo Channels you have already defined in your app with code. 
    #[serde(rename = "existing_android_channel_id", skip_serializing_if = "Option::is_none")]
    pub existing_android_channel_id: Option<String>,
    /// Channel: Push Notifications Platform: Huawei Use this if you have client side Android Oreo Channels you have already defined in your app with code. 
    #[serde(rename = "huawei_existing_channel_id", skip_serializing_if = "Option::is_none")]
    pub huawei_existing_channel_id: Option<String>,
    #[serde(rename = "android_background_layout", skip_serializing_if = "Option::is_none")]
    pub android_background_layout: Option<Box<crate::models::BasicNotificationAllOfAndroidBackgroundLayout>>,
    /// Channel: Push Notifications Platform: Android Icon shown in the status bar and on the top left of the notification. If not set a bell icon will be used or ic_stat_onesignal_default if you have set this resource name. See: How to create small icons 
    #[serde(rename = "small_icon", skip_serializing_if = "Option::is_none")]
    pub small_icon: Option<String>,
    /// Channel: Push Notifications Platform: Huawei Icon shown in the status bar and on the top left of the notification. Use an Android resource path (E.g. /drawable/small_icon). Defaults to your app icon if not set. 
    #[serde(rename = "huawei_small_icon", skip_serializing_if = "Option::is_none")]
    pub huawei_small_icon: Option<String>,
    /// Channel: Push Notifications Platform: Android Can be a drawable resource name or a URL. See: How to create large icons 
    #[serde(rename = "large_icon", skip_serializing_if = "Option::is_none")]
    pub large_icon: Option<String>,
    /// Channel: Push Notifications Platform: Huawei Can be a drawable resource name or a URL. See: How to create large icons 
    #[serde(rename = "huawei_large_icon", skip_serializing_if = "Option::is_none")]
    pub huawei_large_icon: Option<String>,
    /// Channel: Push Notifications Platform: Amazon If not set a bell icon will be used or ic_stat_onesignal_default if you have set this resource name. See: How to create small icons 
    #[serde(rename = "adm_small_icon", skip_serializing_if = "Option::is_none")]
    pub adm_small_icon: Option<String>,
    /// Channel: Push Notifications Platform: Amazon If blank the small_icon is used. Can be a drawable resource name or a URL. See: How to create large icons 
    #[serde(rename = "adm_large_icon", skip_serializing_if = "Option::is_none")]
    pub adm_large_icon: Option<String>,
    /// Channel: Push Notifications Platform: Chrome Sets the web push notification's icon. An image URL linking to a valid image. Common image types are supported; GIF will not animate. We recommend 256x256 (at least 80x80) to display well on high DPI devices. Firefox will also use this icon, unless you specify firefox_icon. 
    #[serde(rename = "chrome_web_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_web_icon: Option<String>,
    /// Channel: Push Notifications Platform: Chrome Sets the web push notification icon for Android devices in the notification shade. Please see Web Push Notification Badge. 
    #[serde(rename = "chrome_web_badge", skip_serializing_if = "Option::is_none")]
    pub chrome_web_badge: Option<String>,
    /// Channel: Push Notifications Platform: Firefox Not recommended Few people need to set Firefox-specific icons. We recommend setting chrome_web_icon instead, which Firefox will also use. Sets the web push notification's icon for Firefox. An image URL linking to a valid image. Common image types are supported; GIF will not animate. We recommend 256x256 (at least 80x80) to display well on high DPI devices. 
    #[serde(rename = "firefox_icon", skip_serializing_if = "Option::is_none")]
    pub firefox_icon: Option<String>,
    /// Channel: Push Notifications Platform: ChromeApp This flag is not used for web push For web push, please see chrome_web_icon instead. The local URL to an icon to use. If blank, the app icon will be used. 
    #[serde(rename = "chrome_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_icon: Option<String>,
    /// Channel: Push Notifications Platform: iOS Sound file that is included in your app to play instead of the default device notification sound. Pass nil to disable vibration and sound for the notification. Example: \"notification.wav\" 
    #[serde(rename = "ios_sound", skip_serializing_if = "Option::is_none")]
    pub ios_sound: Option<String>,
    /// Channel: Push Notifications Platform: Android &#9888;&#65039;Deprecated, this field doesn't work on Android 8 (Oreo) and newer devices! Please use Notification Categories / Channels noted above instead to support ALL versions of Android. Sound file that is included in your app to play instead of the default device notification sound. Pass nil to disable sound for the notification. NOTE: Leave off file extension for Android. Example: \"notification\" 
    #[serde(rename = "android_sound", skip_serializing_if = "Option::is_none")]
    pub android_sound: Option<String>,
    /// Channel: Push Notifications Platform: Huawei &#9888;&#65039;Deprecated, this field ONLY works on EMUI 5 (Android 7 based) and older devices. Please also set Notification Categories / Channels noted above to support EMUI 8 (Android 8 based) devices. Sound file that is included in your app to play instead of the default device notification sound. NOTE: Leave off file extension for and include the full path.  Example: \"/res/raw/notification\" 
    #[serde(rename = "huawei_sound", skip_serializing_if = "Option::is_none")]
    pub huawei_sound: Option<String>,
    /// Channel: Push Notifications Platform: Amazon &#9888;&#65039;Deprecated, this field doesn't work on Android 8 (Oreo) and newer devices! Please use Notification Categories / Channels noted above instead to support ALL versions of Android. Sound file that is included in your app to play instead of the default device notification sound. Pass nil to disable sound for the notification. NOTE: Leave off file extension for Android. Example: \"notification\" 
    #[serde(rename = "adm_sound", skip_serializing_if = "Option::is_none")]
    pub adm_sound: Option<String>,
    /// Channel: Push Notifications Platform: Windows Sound file that is included in your app to play instead of the default device notification sound. Example: \"notification.wav\" 
    #[serde(rename = "wp_wns_sound", skip_serializing_if = "Option::is_none")]
    pub wp_wns_sound: Option<String>,
    /// Channel: Push Notifications Platform: Android &#9888;&#65039;Deprecated, this field doesn't work on Android 8 (Oreo) and newer devices! Please use Notification Categories / Channels noted above instead to support ALL versions of Android. Sets the devices LED notification light if the device has one. ARGB Hex format. Example(Blue): \"FF0000FF\" 
    #[serde(rename = "android_led_color", skip_serializing_if = "Option::is_none")]
    pub android_led_color: Option<String>,
    /// Channel: Push Notifications Platform: Huawei &#9888;&#65039;Deprecated, this field ONLY works on EMUI 5 (Android 7 based) and older devices. Please also set Notification Categories / Channels noted above to support EMUI 8 (Android 8 based) devices. Sets the devices LED notification light if the device has one. RGB Hex format. Example(Blue): \"0000FF\" 
    #[serde(rename = "huawei_led_color", skip_serializing_if = "Option::is_none")]
    pub huawei_led_color: Option<String>,
    /// Channel: Push Notifications Platform: Android Sets the background color of the notification circle to the left of the notification text. Only applies to apps targeting Android API level 21+ on Android 5.0+ devices. Example(Red): \"FFFF0000\" 
    #[serde(rename = "android_accent_color", skip_serializing_if = "Option::is_none")]
    pub android_accent_color: Option<String>,
    /// Channel: Push Notifications Platform: Huawei Accent Color used on Action Buttons and Group overflow count. Uses RGB Hex value (E.g. #9900FF). Defaults to device's theme color if not set. 
    #[serde(rename = "huawei_accent_color", skip_serializing_if = "Option::is_none")]
    pub huawei_accent_color: Option<String>,
    /// Channel: Push Notifications Platform: Android 5.0_ &#9888;&#65039;Deprecated, this field doesn't work on Android 8 (Oreo) and newer devices! Please use Notification Categories / Channels noted above instead to support ALL versions of Android. 1 = Public (default) (Shows the full message on the lock screen unless the user has disabled all notifications from showing on the lock screen. Please consider the user and mark private if the contents are.) 0 = Private (Hides message contents on lock screen if the user set \"Hide sensitive notification content\" in the system settings) -1 = Secret (Notification does not show on the lock screen at all) 
    #[serde(rename = "android_visibility", skip_serializing_if = "Option::is_none")]
    pub android_visibility: Option<i32>,
    /// Channel: Push Notifications Platform: Huawei &#9888;&#65039;Deprecated, this field ONLY works on EMUI 5 (Android 7 based) and older devices. Please also set Notification Categories / Channels noted above to support EMUI 8 (Android 8 based) devices. 1 = Public (default) (Shows the full message on the lock screen unless the user has disabled all notifications from showing on the lock screen. Please consider the user and mark private if the contents are.) 0 = Private (Hides message contents on lock screen if the user set \"Hide sensitive notification content\" in the system settings) -1 = Secret (Notification does not show on the lock screen at all) 
    #[serde(rename = "huawei_visibility", skip_serializing_if = "Option::is_none")]
    pub huawei_visibility: Option<i32>,
    /// Channel: Push Notifications Platform: iOS Describes whether to set or increase/decrease your app's iOS badge count by the ios_badgeCount specified count. Can specify None, SetTo, or Increase. `None` leaves the count unaffected. `SetTo` directly sets the badge count to the number specified in ios_badgeCount. `Increase` adds the number specified in ios_badgeCount to the total. Use a negative number to decrease the badge count. 
    #[serde(rename = "ios_badgeType", skip_serializing_if = "Option::is_none")]
    pub ios_badge_type: Option<String>,
    /// Channel: Push Notifications Platform: iOS Used with ios_badgeType, describes the value to set or amount to increase/decrease your app's iOS badge count by. You can use a negative number to decrease the badge count when used with an ios_badgeType of Increase. 
    #[serde(rename = "ios_badgeCount", skip_serializing_if = "Option::is_none")]
    pub ios_badge_count: Option<i32>,
    /// Channel: Push Notifications Platform: iOS 10+, Android Only one notification with the same id will be shown on the device. Use the same id to update an existing notification instead of showing a new one. Limit of 64 characters. 
    #[serde(rename = "collapse_id", skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    /// Channel: Push Notifications Platform: All Browsers Display multiple notifications at once with different topics. 
    #[serde(rename = "web_push_topic", skip_serializing_if = "Option::is_none")]
    pub web_push_topic: Option<String>,
    /// Channel: Push Notifications Platform: iOS 10+ iOS can localize push notification messages on the client using special parameters such as loc-key. When using the Create Notification endpoint, you must include these parameters inside of a field called apns_alert. Please see Apple's guide on localizing push notifications to learn more. 
    #[serde(rename = "apns_alert", skip_serializing_if = "Option::is_none")]
    pub apns_alert: Option<serde_json::Value>,
    /// Channel: All Possible values are: timezone (Deliver at a specific time-of-day in each users own timezone) last-active Same as Intelligent Delivery . (Deliver at the same time of day as each user last used your app). If send_after is used, this takes effect after the send_after time has elapsed. 
    #[serde(rename = "delayed_option", skip_serializing_if = "Option::is_none")]
    pub delayed_option: Option<String>,
    /// Channel: All Use with delayed_option=timezone. Examples: \"9:00AM\" \"21:45\" \"9:45:30\" 
    #[serde(rename = "delivery_time_of_day", skip_serializing_if = "Option::is_none")]
    pub delivery_time_of_day: Option<String>,
    /// Channel: Push Notifications Platform: iOS, Android, Chrome, Firefox, Safari, ChromeWeb Time To Live - In seconds. The notification will be expired if the device does not come back online within this time. The default is 259,200 seconds (3 days). Max value to set is 2419200 seconds (28 days). 
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// Channel: Push Notifications Platform: Android, Chrome, ChromeWeb Delivery priority through the push server (example GCM/FCM). Pass 10 for high priority or any other integer for normal priority. Defaults to normal priority for Android and high for iOS. For Android 6.0+ devices setting priority to high will wake the device out of doze mode. 
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Channel: Push Notifications Platform: iOS valid values: voip Set the value to voip for sending VoIP Notifications This field maps to the APNS header apns-push-type. Note: alert and background are automatically set by OneSignal 
    #[serde(rename = "apns_push_type_override", skip_serializing_if = "Option::is_none")]
    pub apns_push_type_override: Option<String>,
    /// Channel: All Apps with throttling enabled:   - the parameter value will be used to override the default application throttling value set from the dashboard settings.   - parameter value 0 indicates not to apply throttling to the notification.   - if the parameter is not passed then the default app throttling value will be applied to the notification. Apps with throttling disabled:   - this parameter can be used to throttle delivery for the notification even though throttling is not enabled at the application level. Refer to throttling for more details. 
    #[serde(rename = "throttle_rate_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_rate_per_minute: Option<String>,
    /// Channel: Push Notifications Platform: Android Notifications with the same group will be stacked together using Android's Notification Grouping feature. 
    #[serde(rename = "android_group", skip_serializing_if = "Option::is_none")]
    pub android_group: Option<String>,
    /// Channel: Push Notifications Platform: Android Note: This only works for Android 6 and older. Android 7+ allows full expansion of all message. Summary message to display when 2+ notifications are stacked together. Default is \"# new messages\". Include $[notif_count] in your message and it will be replaced with the current number. Languages - The value of each key is the message that will be sent to users for that language. \"en\" (English) is required. The key of each hash is either a a 2 character language code or one of zh-Hans/zh-Hant for Simplified or Traditional Chinese. Read more: supported languages. Example: {\"en\": \"You have $[notif_count] new messages\"} 
    #[serde(rename = "android_group_message", skip_serializing_if = "Option::is_none")]
    pub android_group_message: Option<String>,
    /// Channel: Push Notifications Platform: Amazon Notifications with the same group will be stacked together using Android's Notification Grouping feature. 
    #[serde(rename = "adm_group", skip_serializing_if = "Option::is_none")]
    pub adm_group: Option<String>,
    /// Channel: Push Notifications Platform: Amazon Summary message to display when 2+ notifications are stacked together. Default is \"# new messages\". Include $[notif_count] in your message and it will be replaced with the current number. \"en\" (English) is required. The key of each hash is either a a 2 character language code or one of zh-Hans/zh-Hant for Simplified or Traditional Chinese. The value of each key is the message that will be sent to users for that language. Example: {\"en\": \"You have $[notif_count] new messages\"} 
    #[serde(rename = "adm_group_message", skip_serializing_if = "Option::is_none")]
    pub adm_group_message: Option<serde_json::Value>,
    /// Channel: Push Notifications Platform: iOS 12+ This parameter is supported in iOS 12 and above. It allows you to group related notifications together. If two notifications have the same thread-id, they will both be added to the same group. 
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// Channel: Push Notifications Platform: iOS 12+ When using thread_id to create grouped notifications in iOS 12+, you can also control the summary. For example, a grouped notification can say \"12 more notifications from John Doe\". The summary_arg lets you set the name of the person/thing the notifications are coming from, and will show up as \"X more notifications from summary_arg\" 
    #[serde(rename = "summary_arg", skip_serializing_if = "Option::is_none")]
    pub summary_arg: Option<String>,
    /// Channel: Push Notifications Platform: iOS 12+ When using thread_id, you can also control the count of the number of notifications in the group. For example, if the group already has 12 notifications, and you send a new notification with summary_arg_count = 2, the new total will be 14 and the summary will be \"14 more notifications from summary_arg\" 
    #[serde(rename = "summary_arg_count", skip_serializing_if = "Option::is_none")]
    pub summary_arg_count: Option<i32>,
    /// Channel: Email Required.  The subject of the email. 
    #[serde(rename = "email_subject", skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    /// Channel: Email Required unless template_id is set. HTML suported The body of the email you wish to send. Typically, customers include their own HTML templates here. Must include [unsubscribe_url] in an <a> tag somewhere in the email. Note: any malformed HTML content will be sent to users. Please double-check your HTML is valid. 
    #[serde(rename = "email_body", skip_serializing_if = "Option::is_none")]
    pub email_body: Option<String>,
    /// Channel: Email The name the email is from. If not specified, will default to \"from name\" set in the OneSignal Dashboard Email Settings. 
    #[serde(rename = "email_from_name", skip_serializing_if = "Option::is_none")]
    pub email_from_name: Option<String>,
    /// Channel: Email The email address the email is from. If not specified, will default to \"from email\" set in the OneSignal Dashboard Email Settings. 
    #[serde(rename = "email_from_address", skip_serializing_if = "Option::is_none")]
    pub email_from_address: Option<String>,
    /// Channel: Email The preheader text of the email. Preheader is the preview text displayed immediately after an email subject that provides additional context about the email content. If not specified, will default to null. 
    #[serde(rename = "email_preheader", skip_serializing_if = "Option::is_none")]
    pub email_preheader: Option<String>,
    /// Channel: Email Default is `false`. This field is used to send transactional notifications. If set to `true`, this notification will also be sent to unsubscribed emails. If a `template_id` is provided, the `include_unsubscribed` value from the template will be inherited. If you are using a third-party ESP, this field requires the ESP's list of unsubscribed emails to be cleared.
    #[serde(rename = "include_unsubscribed", skip_serializing_if = "Option::is_none")]
    pub include_unsubscribed: Option<bool>,
    /// Channel: SMS Phone Number used to send SMS. Should be a registered Twilio phone number in E.164 format. 
    #[serde(rename = "sms_from", skip_serializing_if = "Option::is_none")]
    pub sms_from: Option<String>,
    /// Channel: SMS URLs for the media files to be attached to the SMS content. Limit: 10 media urls with a total max. size of 5MBs. 
    #[serde(rename = "sms_media_urls", skip_serializing_if = "Option::is_none")]
    pub sms_media_urls: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::models::Filter>>,
    /// Channel: All JSON object that can be used as a source of message personalization data for fields that support tag variable substitution. Push, SMS: Can accept up to 2048 bytes of valid JSON. Email: Can accept up to 10000 bytes of valid JSON. Example: {\"order_id\": 123, \"currency\": \"USD\", \"amount\": 25} 
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<serde_json::Value>,
}

impl BasicNotificationAllOf {
    pub fn new() -> BasicNotificationAllOf {
        BasicNotificationAllOf {
            id: None,
            value: None,
            name: None,
            aggregation: None,
            is_ios: None,
            is_android: None,
            is_huawei: None,
            is_any_web: None,
            is_chrome_web: None,
            is_firefox: None,
            is_safari: None,
            is_wp_wns: None,
            is_adm: None,
            is_chrome: None,
            channel_for_external_user_ids: None,
            app_id: None,
            external_id: None,
            idempotency_key: None,
            contents: None,
            headings: None,
            subtitle: None,
            data: None,
            huawei_msg_type: None,
            url: None,
            web_url: None,
            app_url: None,
            ios_attachments: None,
            template_id: None,
            content_available: None,
            mutable_content: None,
            target_content_identifier: None,
            big_picture: None,
            huawei_big_picture: None,
            adm_big_picture: None,
            chrome_big_picture: None,
            chrome_web_image: None,
            buttons: None,
            web_buttons: None,
            ios_category: None,
            android_channel_id: None,
            huawei_channel_id: None,
            existing_android_channel_id: None,
            huawei_existing_channel_id: None,
            android_background_layout: None,
            small_icon: None,
            huawei_small_icon: None,
            large_icon: None,
            huawei_large_icon: None,
            adm_small_icon: None,
            adm_large_icon: None,
            chrome_web_icon: None,
            chrome_web_badge: None,
            firefox_icon: None,
            chrome_icon: None,
            ios_sound: None,
            android_sound: None,
            huawei_sound: None,
            adm_sound: None,
            wp_wns_sound: None,
            android_led_color: None,
            huawei_led_color: None,
            android_accent_color: None,
            huawei_accent_color: None,
            android_visibility: None,
            huawei_visibility: None,
            ios_badge_type: None,
            ios_badge_count: None,
            collapse_id: None,
            web_push_topic: None,
            apns_alert: None,
            delayed_option: None,
            delivery_time_of_day: None,
            ttl: None,
            priority: None,
            apns_push_type_override: None,
            throttle_rate_per_minute: None,
            android_group: None,
            android_group_message: None,
            adm_group: None,
            adm_group_message: None,
            thread_id: None,
            summary_arg: None,
            summary_arg_count: None,
            email_subject: None,
            email_body: None,
            email_from_name: None,
            email_from_address: None,
            email_preheader: None,
            include_unsubscribed: None,
            sms_from: None,
            sms_media_urls: None,
            filters: None,
            custom_data: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AggregationType {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "count")]
    Count,
}

impl Default for AggregationType {
    fn default() -> AggregationType {
        Self::Sum
    }
}

