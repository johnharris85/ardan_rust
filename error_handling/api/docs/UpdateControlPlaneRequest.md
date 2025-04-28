# UpdateControlPlaneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the control plane. | [optional]
**description** | Option<**String**> | The description of the control plane in Konnect. | [optional]
**auth_type** | Option<**String**> | The auth type value of the cluster associated with the Runtime Group. | [optional]
**proxy_urls** | Option<[**Vec<models::ProxyUrl>**](ProxyURL.md)> | Array of proxy URLs associated with reaching the data-planes connected to a control-plane. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels to facilitate tagged search on control planes. Keys must be of length 1-63 characters, and cannot start with 'kong', 'konnect', 'mesh', 'kic', or '_'. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


