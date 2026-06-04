# GetSegmentsSuccessResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_count** | Option<**i32**> | The number of Segments in the response. | [optional]
**offset** | Option<**i32**> | Set with the offset query parameter. Default 0. | [optional]
**limit** | Option<**i32**> | Maximum number of Segments returned. Default 300. | [optional]
**segments** | Option<[**Vec<crate::models::SegmentData>**](SegmentData.md)> | An array containing the Segment information. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


