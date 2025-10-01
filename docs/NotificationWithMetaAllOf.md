# NotificationWithMetaAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**remaining** | Option<**i32**> | Number of notifications that have not been sent out yet. This can mean either our system is still processing the notification or you have delayed options set. | [optional]
**successful** | Option<**i32**> | Number of notifications that were successfully delivered. | [optional]
**failed** | Option<**i32**> | Number of notifications that could not be delivered due to those devices being unsubscribed. | [optional]
**errored** | Option<**i32**> | Number of notifications that could not be delivered due to an error. You can find more information by viewing the notification in the dashboard. | [optional]
**converted** | Option<**i32**> | Number of users who have clicked / tapped on your notification. | [optional]
**queued_at** | Option<**i64**> | Unix timestamp indicating when the notification was created. | [optional]
**send_after** | Option<**i64**> | Unix timestamp indicating when notification delivery should begin. | [optional]
**completed_at** | Option<**i64**> | Unix timestamp indicating when notification delivery completed. The delivery duration from start to finish can be calculated with completed_at - send_after. | [optional]
**platform_delivery_stats** | Option<[**crate::models::PlatformDeliveryData**](PlatformDeliveryData.md)> |  | [optional]
**received** | Option<**i32**> | Confirmed Deliveries number of devices that received the push notification. Paid Feature Only. Free accounts will see 0. | [optional]
**throttle_rate_per_minute** | Option<**i32**> | number of push notifications sent per minute. Paid Feature Only. If throttling is not enabled for the app or the notification, and for free accounts, null is returned. Refer to Throttling for more details. | [optional]
**canceled** | Option<**bool**> | Indicates whether the notification was canceled before it could be sent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


