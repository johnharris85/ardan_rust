# BaseError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **i32** | The HTTP status code of the error. Useful when passing the response body to child properties in a frontend UI. Must be returned as an integer.  | [readonly]
**title** | **String** | A short, human-readable summary of the problem. It should not change between occurences of a problem, except for localization. Should be provided as \"Sentence case\" for direct use in the UI.  | [readonly]
**r#type** | Option<**String**> | The error type. | [optional][readonly]
**instance** | **String** | Used to return the correlation ID back to the user, in the format kong:trace:<correlation_id>. This helps us find the relevant logs when a customer reports an issue.  | [readonly]
**detail** | **String** | A human readable explanation specific to this occurence of the problem. This field may contain request/entity data to help the user understand what went wrong. Enclose variable values in square brackets. Should be provided as \"Sentence case\" for direct use in the UI.  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


