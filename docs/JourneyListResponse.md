# JourneyListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**journeys** | Option<[**Vec<crate::models::JourneyListItem>**](JourneyListItem.md)> | Journeys ordered by creation time, newest first. | [optional]
**has_more** | Option<**bool**> | true if more journeys exist beyond this page. | [optional]
**next_cursor** | Option<**String**> | Cursor for the next page. Present only when has_more is true. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


