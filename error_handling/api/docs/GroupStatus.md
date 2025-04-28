# GroupStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The control plane group ID. | [readonly]
**created_at** | **String** | An ISO-8604 timestamp representation of control plane group status creation date. | [readonly]
**updated_at** | **String** | An ISO-8604 timestamp representation of control plane group status update date. | [readonly]
**conflicts** | Option<[**Vec<models::GroupConflict>**](GroupConflict.md)> |  | [optional]
**state** | **String** | The state of the control plane group. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


