# NotificationSlice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_count** | Option<**i32**> |  | [optional]
**offset** | Option<**i32**> |  | [optional]
**limit** | Option<**i32**> |  | [optional]
**time_offset** | Option<**String**> | The time_offset cursor specified in the request, if any. | [optional]
**next_time_offset** | Option<**String**> | An opaque Base64 cursor token representing the next page of messages to fetch.  Present when time_offset was provided in the request.  Pass this value as time_offset on the next request to continue paginating. | [optional]
**notifications** | Option<[**Vec<crate::models::NotificationWithMeta>**](NotificationWithMeta.md)> |  | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


