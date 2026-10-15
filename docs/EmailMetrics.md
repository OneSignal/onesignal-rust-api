# EmailMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sent** | Option<**i64**> | The number of emails sent. | [optional]
**delivered** | Option<**i64**> | The number of emails successfully delivered. | [optional]
**opened** | Option<**i64**> | The total number of email opens, including repeat opens by the same recipient. | [optional]
**clicked** | Option<**i64**> | The total number of email link clicks, including repeat clicks by the same recipient. | [optional]
**unique_opened** | Option<**i64**> | The number of emails opened at least once. | [optional]
**unique_clicked** | Option<**i64**> | The number of emails with at least one link click. | [optional]
**failed** | Option<**i64**> | The number of emails that failed to send. | [optional]
**bounced** | Option<**i64**> | The number of emails that bounced. | [optional]
**complained** | Option<**i64**> | The number of emails that recipients reported as spam. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


