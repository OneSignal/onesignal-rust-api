# SegmentNotificationTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**included_segments** | Option<**Vec<String>**> | The segment names you want to target. Users in these segments will receive a notification. This targeting parameter is only compatible with excluded_segments. Example: [\"Active Users\", \"Inactive Users\"] `\"All\"` is a shorthand for every subscribed user: if the array includes the string `\"All\"` and the app has no segment actually named `All`, it targets all subscribers instead of a literal segment lookup.  | [optional]
**excluded_segments** | Option<**Vec<String>**> | Segment that will be excluded when sending. Users in these segments will not receive a notification, even if they were included in included_segments. This targeting parameter is only compatible with included_segments. Example: [\"Active Users\", \"Inactive Users\"]  | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


