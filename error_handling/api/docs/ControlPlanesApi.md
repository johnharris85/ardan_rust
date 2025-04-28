# \ControlPlanesApi

All URIs are relative to *https://us.api.konghq.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_control_plane**](ControlPlanesApi.md#create_control_plane) | **POST** /control-planes | Create Control Plane
[**delete_control_plane**](ControlPlanesApi.md#delete_control_plane) | **DELETE** /control-planes/{id} | Delete Control Plane
[**get_control_plane**](ControlPlanesApi.md#get_control_plane) | **GET** /control-planes/{id} | Fetch Control Plane
[**list_control_planes**](ControlPlanesApi.md#list_control_planes) | **GET** /control-planes | List Control Planes
[**update_control_plane**](ControlPlanesApi.md#update_control_plane) | **PATCH** /control-planes/{id} | Update Control Plane



## create_control_plane

> models::ControlPlane create_control_plane(create_control_plane_request)
Create Control Plane

Create a control plane in the Konnect Organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_control_plane_request** | [**CreateControlPlaneRequest**](CreateControlPlaneRequest.md) |  | [required] |

### Return type

[**models::ControlPlane**](ControlPlane.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_control_plane

> delete_control_plane(id)
Delete Control Plane

Delete an individual control plane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The control plane ID | [required] |

### Return type

 (empty response body)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_control_plane

> models::ControlPlane get_control_plane(id)
Fetch Control Plane

Returns information about an individual control plane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The control plane ID | [required] |

### Return type

[**models::ControlPlane**](ControlPlane.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_control_planes

> models::ListControlPlanesResponse list_control_planes(page_left_square_bracket_size_right_square_bracket, page_left_square_bracket_number_right_square_bracket, filter, labels, sort)
List Control Planes

Returns an array of control plane objects containing information about the Konnect Control Planes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_size_right_square_bracket** | Option<**i32**> | The maximum number of items to include per page. The last page of a collection may include fewer items. |  |
**page_left_square_bracket_number_right_square_bracket** | Option<**i32**> | Determines which page of the entities to retrieve. |  |
**filter** | Option<[**ControlPlaneFilterParameters**](.md)> | Filters a collection of control-planes. |  |
**labels** | Option<**String**> | Filter control planes in the response by associated labels. |  |
**sort** | Option<**String**> | Sorts a collection of control-planes. Supported sort attributes are:   - created_at  |  |

### Return type

[**models::ListControlPlanesResponse**](ListControlPlanesResponse.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_control_plane

> models::ControlPlane update_control_plane(id, update_control_plane_request)
Update Control Plane

Update an individual control plane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The control plane ID | [required] |
**update_control_plane_request** | [**UpdateControlPlaneRequest**](UpdateControlPlaneRequest.md) |  | [required] |

### Return type

[**models::ControlPlane**](ControlPlane.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

