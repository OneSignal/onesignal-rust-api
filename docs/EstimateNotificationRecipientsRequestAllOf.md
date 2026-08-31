# EstimateNotificationRecipientsRequestAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_id** | Option<**String**> | The OneSignal App ID for your app, which can be found in Keys & IDs. | [optional]
**filters** | Option<[**Vec<crate::models::FilterExpression>**](FilterExpression.md)> |  | [optional]
**include_aliases** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | Target specific users by aliases assigned via API. An alias can be an external_id, onesignal_id, or a custom alias. Accepts an object where keys are alias labels and values are arrays of alias IDs to include Example usage: { \"external_id\": [\"exId1\", \"extId2\"], \"internal_label\": [\"id1\", \"id2\"] } Keys must match API spellings exactly (for example the label for External ID is the string `external_id`; arbitrary keys such as camelCase variants are not aliases and may yield no recipients). Not compatible with any other targeting parameters. REQUIRED: REST API Key Authentication Limit of 2,000 entries per REST API call Note: If targeting push, email, or sms subscribers with same ids, use with target_channel to indicate you are sending a push or email or sms. | [optional]
**target_channel** | Option<**String**> | Which platforms to count recipients for. Selects the same default platforms Create notification would use for the channel. Individual platform flags (`isIos`, `isAndroid`, etc.) are not supported by this endpoint. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


