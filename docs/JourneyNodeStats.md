# JourneyNodeStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | Node kind, repeated here so stats can be read without joining against the journey definition. | [optional]
**waiting** | Option<**i32**> | Users currently held at this node. | [optional]
**completed** | Option<**i32**> | Users who advanced past this node normally. | [optional]
**exited_early** | Option<**i32**> | Users who left the journey from this node through an early exit rule. | [optional]
**message_stats** | Option<[**crate::models::JourneyMessageStats**](JourneyMessageStats.md)> |  | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


