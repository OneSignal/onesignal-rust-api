# GetNotificationRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | Option<**String**> | -> \"sent\" - All the devices by player_id that were sent the specified notification_id.  Notifications targeting under 1000 recipients will not have \"sent\" events recorded, but will show \"clicked\" events. \"clicked\" - All the devices by `player_id` that clicked the specified notification_id. | [optional]
**email** | Option<**String**> | The email address you would like the report sent. | [optional]
**app_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


