# Filter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | Option<**String**> | Required. Name of the field to use as the first operand in the filter expression. | [optional]
**key** | Option<**String**> | If `field` is `tag`, this field is *required* to specify `key` inside the tags. | [optional]
**value** | Option<**String**> | Constant value to use as the second operand in the filter expression. This value is *required* when the relation operator is a binary operator. | [optional]
**hours_ago** | Option<**String**> | If `field` is session-related, this is *required* to specify the number of hours before or after the user's session. | [optional]
**radius** | Option<**f32**> | If `field` is `location`, this will specify the radius in meters from a provided location point. Use with `lat` and `long`. | [optional]
**lat** | Option<**f32**> | If `field` is `location`, this is *required* to specify the user's latitude. | [optional]
**long** | Option<**f32**> | If `field` is `location`, this is *required* to specify the user's longitude. | [optional]
**relation** | Option<**String**> | Required. Operator of a filter expression. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


