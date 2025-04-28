# \ControlPlaneGroupsApi

All URIs are relative to *https://us.api.konghq.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_control_planes_id_group_member_status**](ControlPlaneGroupsApi.md#get_control_planes_id_group_member_status) | **GET** /control-planes/{id}/group-member-status | Control Plane Group Member Status
[**get_control_planes_id_group_memberships**](ControlPlaneGroupsApi.md#get_control_planes_id_group_memberships) | **GET** /control-planes/{id}/group-memberships | List Control Plane Group Memberships
[**get_control_planes_id_group_status**](ControlPlaneGroupsApi.md#get_control_planes_id_group_status) | **GET** /control-planes/{id}/group-status | Get Control Plane Group Status
[**post_control_planes_id_group_memberships_add**](ControlPlaneGroupsApi.md#post_control_planes_id_group_memberships_add) | **POST** /control-planes/{id}/group-memberships/add | Add Control Plane Group Members
[**post_control_planes_id_group_memberships_remove**](ControlPlaneGroupsApi.md#post_control_planes_id_group_memberships_remove) | **POST** /control-planes/{id}/group-memberships/remove | Remove Control Plane Group Members
[**put_control_planes_id_group_memberships**](ControlPlaneGroupsApi.md#put_control_planes_id_group_memberships) | **PUT** /control-planes/{id}/group-memberships | Upsert Control Plane Group Members



## get_control_planes_id_group_member_status

> models::GroupMemberStatus get_control_planes_id_group_member_status(id)
Control Plane Group Member Status

Determines the group membership status of a control plane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of a control plane | [required] |

### Return type

[**models::GroupMemberStatus**](GroupMemberStatus.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_control_planes_id_group_memberships

> models::GetControlPlanesIdGroupMemberships200Response get_control_planes_id_group_memberships(id, page_left_square_bracket_size_right_square_bracket, page_left_square_bracket_after_right_square_bracket)
List Control Plane Group Memberships

Returns an array of control planes that are a member of this control plane group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of a control plane group | [required] |
**page_left_square_bracket_size_right_square_bracket** | Option<**i32**> | The maximum number of items to include per page. The last page of a collection may include fewer items. |  |
**page_left_square_bracket_after_right_square_bracket** | Option<**String**> | Request the next page of data, starting with the item after this parameter. |  |

### Return type

[**models::GetControlPlanesIdGroupMemberships200Response**](get_control_planes_id_group_memberships_200_response.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_control_planes_id_group_status

> models::GetControlPlanesIdGroupStatus200Response get_control_planes_id_group_status(id)
Get Control Plane Group Status

Returns the status of a control plane group, including existing conflicts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of a control plane group | [required] |

### Return type

[**models::GetControlPlanesIdGroupStatus200Response**](get_control_planes_id_group_status_200_response.md)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_control_planes_id_group_memberships_add

> post_control_planes_id_group_memberships_add(id, group_membership)
Add Control Plane Group Members

Adds one or more control planes as a member of a control plane group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of a control plane group | [required] |
**group_membership** | Option<[**GroupMembership**](GroupMembership.md)> | Request body for adding a list of child control planes to a control plane group membership. |  |

### Return type

 (empty response body)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_control_planes_id_group_memberships_remove

> post_control_planes_id_group_memberships_remove(id, group_membership)
Remove Control Plane Group Members

Removes one or more control planes from the members of a control plane group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of a control plane group | [required] |
**group_membership** | Option<[**GroupMembership**](GroupMembership.md)> | Request body for removing a list of child control planes from a control plane group membership. |  |

### Return type

 (empty response body)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_control_planes_id_group_memberships

> put_control_planes_id_group_memberships(id, group_membership)
Upsert Control Plane Group Members

Adds one or more control planes as a member of a control plane group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of a control plane group | [required] |
**group_membership** | Option<[**GroupMembership**](GroupMembership.md)> | Request body for upserting a list of child control planes to a control plane group membership. |  |

### Return type

 (empty response body)

### Authorization

[systemAccountAccessToken](../README.md#systemAccountAccessToken), [personalAccessToken](../README.md#personalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

