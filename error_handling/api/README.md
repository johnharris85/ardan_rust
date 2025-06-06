# Rust API client for openapi

The API for Kong Konnect Control Planes.

For more information, please visit [https://cloud.konghq.com](https://cloud.konghq.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0.0
- Package version: 2.0.0
- Generator version: 7.12.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://us.api.konghq.com/v2*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ControlPlaneGroupsApi* | [**get_control_planes_id_group_member_status**](docs/ControlPlaneGroupsApi.md#get_control_planes_id_group_member_status) | **GET** /control-planes/{id}/group-member-status | Control Plane Group Member Status
*ControlPlaneGroupsApi* | [**get_control_planes_id_group_memberships**](docs/ControlPlaneGroupsApi.md#get_control_planes_id_group_memberships) | **GET** /control-planes/{id}/group-memberships | List Control Plane Group Memberships
*ControlPlaneGroupsApi* | [**get_control_planes_id_group_status**](docs/ControlPlaneGroupsApi.md#get_control_planes_id_group_status) | **GET** /control-planes/{id}/group-status | Get Control Plane Group Status
*ControlPlaneGroupsApi* | [**post_control_planes_id_group_memberships_add**](docs/ControlPlaneGroupsApi.md#post_control_planes_id_group_memberships_add) | **POST** /control-planes/{id}/group-memberships/add | Add Control Plane Group Members
*ControlPlaneGroupsApi* | [**post_control_planes_id_group_memberships_remove**](docs/ControlPlaneGroupsApi.md#post_control_planes_id_group_memberships_remove) | **POST** /control-planes/{id}/group-memberships/remove | Remove Control Plane Group Members
*ControlPlaneGroupsApi* | [**put_control_planes_id_group_memberships**](docs/ControlPlaneGroupsApi.md#put_control_planes_id_group_memberships) | **PUT** /control-planes/{id}/group-memberships | Upsert Control Plane Group Members
*ControlPlanesApi* | [**create_control_plane**](docs/ControlPlanesApi.md#create_control_plane) | **POST** /control-planes | Create Control Plane
*ControlPlanesApi* | [**delete_control_plane**](docs/ControlPlanesApi.md#delete_control_plane) | **DELETE** /control-planes/{id} | Delete Control Plane
*ControlPlanesApi* | [**get_control_plane**](docs/ControlPlanesApi.md#get_control_plane) | **GET** /control-planes/{id} | Fetch Control Plane
*ControlPlanesApi* | [**list_control_planes**](docs/ControlPlanesApi.md#list_control_planes) | **GET** /control-planes | List Control Planes
*ControlPlanesApi* | [**update_control_plane**](docs/ControlPlanesApi.md#update_control_plane) | **PATCH** /control-planes/{id} | Update Control Plane


## Documentation For Models

 - [BadRequestError](docs/BadRequestError.md)
 - [BaseError](docs/BaseError.md)
 - [ConflictError](docs/ConflictError.md)
 - [ControlPlane](docs/ControlPlane.md)
 - [ControlPlaneConfig](docs/ControlPlaneConfig.md)
 - [ControlPlaneFilterParameters](docs/ControlPlaneFilterParameters.md)
 - [ControlPlaneFilterParametersClusterType](docs/ControlPlaneFilterParametersClusterType.md)
 - [ControlPlaneFilterParametersId](docs/ControlPlaneFilterParametersId.md)
 - [ControlPlaneFilterParametersName](docs/ControlPlaneFilterParametersName.md)
 - [CreateControlPlaneRequest](docs/CreateControlPlaneRequest.md)
 - [CursorMetaWithSizeAndTotal](docs/CursorMetaWithSizeAndTotal.md)
 - [CursorPaginatedMetaWithSizeAndTotal](docs/CursorPaginatedMetaWithSizeAndTotal.md)
 - [ForbiddenError](docs/ForbiddenError.md)
 - [GetControlPlanesIdGroupMemberships200Response](docs/GetControlPlanesIdGroupMemberships200Response.md)
 - [GetControlPlanesIdGroupStatus200Response](docs/GetControlPlanesIdGroupStatus200Response.md)
 - [GroupConflict](docs/GroupConflict.md)
 - [GroupConflictResource](docs/GroupConflictResource.md)
 - [GroupMemberStatus](docs/GroupMemberStatus.md)
 - [GroupMembership](docs/GroupMembership.md)
 - [GroupMembershipMembersInner](docs/GroupMembershipMembersInner.md)
 - [GroupStatus](docs/GroupStatus.md)
 - [InternalServerError](docs/InternalServerError.md)
 - [InvalidParameterChoiceItem](docs/InvalidParameterChoiceItem.md)
 - [InvalidParameterDependentItem](docs/InvalidParameterDependentItem.md)
 - [InvalidParameterMaximumLength](docs/InvalidParameterMaximumLength.md)
 - [InvalidParameterMinimumLength](docs/InvalidParameterMinimumLength.md)
 - [InvalidParameterStandard](docs/InvalidParameterStandard.md)
 - [InvalidParametersInner](docs/InvalidParametersInner.md)
 - [InvalidRules](docs/InvalidRules.md)
 - [ListControlPlanesResponse](docs/ListControlPlanesResponse.md)
 - [NotFoundError](docs/NotFoundError.md)
 - [PageMeta](docs/PageMeta.md)
 - [PaginatedMeta](docs/PaginatedMeta.md)
 - [ProxyUrl](docs/ProxyUrl.md)
 - [ServiceUnavailableError](docs/ServiceUnavailableError.md)
 - [StringFieldContainsFilter](docs/StringFieldContainsFilter.md)
 - [StringFieldEqualsComparison](docs/StringFieldEqualsComparison.md)
 - [StringFieldEqualsFilter](docs/StringFieldEqualsFilter.md)
 - [StringFieldNeqFilter](docs/StringFieldNeqFilter.md)
 - [StringFieldOeqFilter](docs/StringFieldOeqFilter.md)
 - [UnauthorizedError](docs/UnauthorizedError.md)
 - [UpdateControlPlaneRequest](docs/UpdateControlPlaneRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



