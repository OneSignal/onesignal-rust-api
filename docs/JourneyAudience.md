# JourneyAudience

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | **String** | Audience kind. Selects which other fields apply. | 
**included_segment_ids** | Option<**Vec<String>**> | segment audiences: Segment UUIDs whose users enter the journey. | [optional]
**excluded_segment_ids** | Option<**Vec<String>**> | segment audiences: Segment UUIDs whose users are excluded. | [optional]
**future_additions_only** | Option<**bool**> | segment audiences: when true, only users who newly match the segment after activation enter the journey. Defaults to false. | [optional]
**name** | Option<**String**> | event_trigger audiences: event name that triggers entry, up to 255 characters. | [optional]
**attributes** | Option<[**Vec<Vec<crate::models::JourneyEventAttribute>>**](array.md)> | Event attribute matchers, as a list of condition groups. Send a single group whose conditions are AND'd together. More than one group is rejected. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


