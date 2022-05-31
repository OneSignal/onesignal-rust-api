# FilterExpressions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Name of the field to use as the first operand in the filter expression. | 
**key** | Option<**String**> | If `field` is `tag`, this field is *required* to specify `key` inside the tags. | [optional]
**value** | Option<**String**> | Constant value to use as the second operand in the filter expression.  This value is *required* when the relation operator is a binary operator. | [optional]
**relation** | **String** | Operator of a filter expression. | 
**operator** | Option<**String**> | Strictly, this must be either `\"OR\"`, or `\"AND\"`.  It can be used to compose Filters as part of a Filters object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


