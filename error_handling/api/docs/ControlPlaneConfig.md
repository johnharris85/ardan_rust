# ControlPlaneConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**control_plane_endpoint** | **String** | Control Plane Endpoint. | [readonly]
**telemetry_endpoint** | **String** | Telemetry Endpoint. | [readonly]
**cluster_type** | **String** | The ClusterType value of the cluster associated with the Control Plane. | [readonly]
**auth_type** | **String** | The auth type value of the cluster associated with the Runtime Group. | [readonly]
**cloud_gateway** | **bool** | Whether the Control Plane can be used for cloud-gateways. | [readonly]
**proxy_urls** | Option<[**Vec<models::ProxyUrl>**](ProxyURL.md)> | Array of proxy URLs associated with reaching the data-planes connected to a control-plane. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


