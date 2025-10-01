# StartLiveActivityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | An internal name to assist with your campaign organization. This does not get displayed in the message itself. | 
**event** | **String** |  | [default to Event_Start]
**activity_id** | **String** | Set a unique activity_id to track and manage the Live Activity. | 
**event_attributes** | [**serde_json::Value**](.md) | Default/static data to initialize the Live Activity upon start. | 
**event_updates** | [**serde_json::Value**](.md) | Dynamic content used to update the running Live Activity at start. Must match the ContentState interface defined in your app. | 
**contents** | [**crate::models::LanguageStringMap**](LanguageStringMap.md) |  | 
**headings** | [**crate::models::LanguageStringMap**](LanguageStringMap.md) |  | 
**stale_date** | Option<**i32**> | Accepts Unix timestamp in seconds. When time reaches the configured stale date, the system considers the Live Activity out of date, and the ActivityState of the Live Activity changes to ActivityState.stale. | [optional]
**priority** | Option<**i32**> | Delivery priority through the push provider (APNs). Pass 10 for higher priority notifications, or 5 for lower priority notifications. Lower priority notifications are sent based on the power considerations of the end user's device. If not set, defaults to 10. | [optional]
**ios_relevance_score** | Option<**f32**> | iOS 15+. A score to indicate how a notification should be displayed when grouped. Use a float between 0-1. | [optional]
**idempotency_key** | Option<**String**> | Correlation and idempotency key. A request received with this parameter will first look for another notification with the same idempotency key. If one exists, a notification will not be sent, and result of the previous operation will instead be returned. Therefore, if you plan on using this feature, it's important to use a good source of randomness to generate the UUID passed here. This key is only idempotent for 30 days. After 30 days, the notification could be removed from our system and a notification with the same idempotency key will be sent again.   See Idempotent Notification Requests for more details writeOnly: true  | [optional]
**include_aliases** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | Target specific users by aliases assigned via API. An alias can be an external_id, onesignal_id, or a custom alias. Accepts an object where keys are alias labels and values are arrays of alias IDs to include Example usage: { \"external_id\": [\"exId1\", \"extId2\"], \"internal_label\": [\"id1\", \"id2\"] } Not compatible with any other targeting parameters. REQUIRED: REST API Key Authentication Limit of 2,000 entries per REST API call Note: If targeting push, email, or sms subscribers with same ids, use with target_channel to indicate you are sending a push or email or sms. | [optional]
**include_subscription_ids** | Option<**Vec<String>**> | Specific subscription ids to target. Not compatible with other targeting parameters. | [optional]
**included_segments** | Option<**Vec<String>**> | Segment names to include. Only compatible with excluded_segments. | [optional]
**excluded_segments** | Option<**Vec<String>**> | Segment names to exclude. Only compatible with included_segments. | [optional]
**filters** | Option<[**Vec<crate::models::FilterExpression>**](FilterExpression.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


