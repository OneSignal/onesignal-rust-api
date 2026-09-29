# DuplicateJourneyOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name for the copy, up to 300 characters. If you omit it, the copy takes the name of the source plus \" (Copy)\". | [optional]
**description** | Option<**String**> | Optional journey description, up to 1024 characters. If you omit it, the copy takes the description of the source. Send null to clear it. | [optional]
**audience** | Option<[**crate::models::JourneyAudience**](JourneyAudience.md)> |  | [optional]
**early_exit** | Option<[**crate::models::JourneyEarlyExit**](JourneyEarlyExit.md)> |  | [optional]
**reentry_rules** | Option<[**crate::models::JourneyReentryRules**](JourneyReentryRules.md)> |  | [optional]
**schedule** | Option<[**crate::models::JourneySchedule**](JourneySchedule.md)> |  | [optional]
**nodes** | Option<[**Vec<crate::models::JourneyNode>**](JourneyNode.md)> | Full ordered list of nodes. Replaces the copied graph. Server-assigned id fields are rejected. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


