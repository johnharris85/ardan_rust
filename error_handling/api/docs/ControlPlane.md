# ControlPlane

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The control plane ID. | [readonly]
**name** | **String** | The name of the control plane. | 
**description** | Option<**String**> | The description of the control plane in Konnect. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels to facilitate tagged search on control planes. Keys must be of length 1-63 characters, and cannot start with 'kong', 'konnect', 'mesh', 'kic', or '_'. | [optional]
**config** | [**models::ControlPlaneConfig**](ControlPlane_config.md) |  | 
**created_at** | **String** | An ISO-8604 timestamp representation of control plane creation date. | [readonly]
**updated_at** | **String** | An ISO-8604 timestamp representation of control plane update date. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


