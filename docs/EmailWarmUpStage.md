# EmailWarmUpStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start** | **String** | ISO 8601 timestamp for the start of this stage. Sending for this stage will not begin before this time. | 
**end** | **String** | ISO 8601 timestamp for the end of this stage. This stage's quota is expected to be sent by this time. | 
**quota** | **i32** | Number of emails to send during this stage. | 
**acked** | Option<**bool**> | Whether this stage has been picked up and acknowledged by the warm-up scheduler. Not accepted on create. This is only present when reading back a campaign. | [optional][readonly]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


