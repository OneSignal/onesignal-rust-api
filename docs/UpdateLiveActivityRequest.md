# UpdateLiveActivityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | An internal name to assist with your campaign organization. This does not get displayed in the message itself. | 
**event** | **String** |  | 
**event_updates** | [**serde_json::Value**](.md) | This must match the ContentState interface you have defined within your Live Activity in your app. | 
**contents** | Option<[**crate::models::LanguageStringMap**](LanguageStringMap.md)> |  | [optional]
**headings** | Option<[**crate::models::LanguageStringMap**](LanguageStringMap.md)> |  | [optional]
**sound** | Option<**String**> | Deprecated. The API ignores this field. Use `ios_sound`. | [optional]
**ios_sound** | Option<**String**> | Sound file that is included in your app to play instead of the default device notification sound. Omit to disable vibration and sound for the notification. Requires `headings` on the same request: ActivityKit ignores an update whose alert has no title, which silently drops the sound. Supersedes the deprecated `sound` field.  | [optional]
**stale_date** | Option<**i32**> | Accepts Unix timestamp in seconds. When time reaches the configured stale date, the system considers the Live Activity out of date, and the ActivityState of the Live Activity changes to ActivityState.stale. | [optional]
**dismissal_date** | Option<**i32**> | Accepts Unix timestamp in seconds; only allowed if event is \"end\" | [optional]
**priority** | Option<**i32**> | Delivery priority through the push provider (APNs). Pass 10 for higher priority notifications, or 5 for lower priority notifications. Lower priority notifications are sent based on the power considerations of the end user's device. If not set, defaults to 10. Some providers (APNs) allow for a limited budget of high priority notifications per hour, and if that budget is exceeded, the provider may throttle notification delivery. | [optional]
**ios_relevance_score** | Option<**f32**> | A value between 0 and 1. When more than one Live Activity is active for your app, the one with the highest relevance score shows in the Dynamic Island. If the scores are equal, the system shows the Live Activity that started first. The score also sets the order of Live Activities on the Lock Screen. Only available on iOS 16.2 and later. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


