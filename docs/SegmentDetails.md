# SegmentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for the segment (UUID v4). | [optional]
**name** | Option<**String**> | The segment name. | [optional]
**description** | Option<**String**> | Human-readable description for the segment. `null` when unset. Maximum 255 characters. | [optional]
**created_at** | Option<**i32**> | Unix timestamp when the segment was created. | [optional]
**source** | Option<**String**> | The source of the segment. | [optional]
**filters** | Option<[**Vec<crate::models::FilterExpression>**](FilterExpression.md)> | Array of filter and operator objects defining the segment criteria. Uses the same format as the Create Segment API, so filters can be directly used to recreate or update the segment. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


