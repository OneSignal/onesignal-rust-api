# CreateJourneyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Journey name, up to 300 characters. | 
**description** | Option<**String**> | Optional journey description, up to 1024 characters. | [optional]
**audience** | Option<[**crate::models::JourneyAudience**](JourneyAudience.md)> |  | [optional]
**early_exit** | Option<[**crate::models::JourneyEarlyExit**](JourneyEarlyExit.md)> |  | [optional]
**reentry_rules** | Option<[**crate::models::JourneyReentryRules**](JourneyReentryRules.md)> |  | [optional]
**schedule** | Option<[**crate::models::JourneySchedule**](JourneySchedule.md)> |  | [optional]
**nodes** | Option<[**Vec<crate::models::JourneyNode>**](JourneyNode.md)> | Ordered list of journey nodes. Server-assigned id fields are rejected on create. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


