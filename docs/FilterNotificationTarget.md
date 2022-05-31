# FilterNotificationTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_session** | Option<**String**> | relation = \">\" or \"<\" hours_ago = number of hours before or after the users last session. Example: \"1.1\"  | [optional]
**first_session** | Option<**String**> | relation = \">\" or \"<\" hours_ago = number of hours before or after the users first session. Example: \"1.1\"  | [optional]
**session_count** | Option<**String**> | relation = \">\", \"<\", \"=\" or \"!=\" value = number sessions. Example: \"1\"  | [optional]
**session_time** | Option<**String**> | relation = \">\", \"<\", \"=\" or \"!=\" value = Time in seconds the user has been in your app. Example: \"3600\"  | [optional]
**amount_spent** | Option<**String**> | relation = \">\", \"<\", or \"=\" value = Amount in USD a user has spent on IAP (In App Purchases). Example: \"0.99\"  | [optional]
**bought_sku** | Option<**String**> | relation = \">\", \"<\" or \"=\" key = SKU purchased in your app as an IAP (In App Purchases). Example: \"com.domain.100coinpack\" value = value of SKU to compare to. Example: \"0.99\"  | [optional]
**tag** | Option<**String**> | relation = \">\", \"<\", \"=\", \"!=\", \"exists\", \"not_exists\", \"time_elapsed_gt\" (paid plan only) or \"time_elapsed_lt\" (paid plan only) See Time Operators key = Tag key to compare. value = Tag value to compare. Not required for \"exists\" or \"not_exists\". Example: See Formatting Filters  | [optional]
**language** | Option<**String**> | relation = \"=\" or \"!=\" value = 2 character language code. Example: \"en\". For a list of all language codes see Language & Localization.  | [optional]
**app_version** | Option<**String**> | relation = \">\", \"<\", \"=\" or \"!=\" value = app version. Example: \"1.0.0\"  | [optional]
**location** | Option<**String**> | radius = in meters lat = latitude long = longitude  | [optional]
**email** | Option<**String**> | value = email address Only for sending Push Notifications Use this for targeting push subscribers associated with an email set with all SDK setEmail methods To send emails to specific email addresses use include_email_tokens parameter  | [optional]
**country** | Option<**String**> | relation = \"=\" value = 2-digit Country code Example: \"field\": \"country\", \"relation\": \"=\", \"value\", \"US\"  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


