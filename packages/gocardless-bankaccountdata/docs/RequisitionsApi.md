# \RequisitionsApi

All URIs are relative to *https://bankaccountdata.gocardless.com*

| Method                                                                        | HTTP request                          | Description |
| ----------------------------------------------------------------------------- | ------------------------------------- | ----------- |
| [**create_requisition**](RequisitionsApi.md#create_requisition)               | **POST** /api/v2/requisitions/        |
| [**delete_requisition_by_id**](RequisitionsApi.md#delete_requisition_by_id)   | **DELETE** /api/v2/requisitions/{id}/ |
| [**requisition_by_id**](RequisitionsApi.md#requisition_by_id)                 | **GET** /api/v2/requisitions/{id}/    |
| [**retrieve_all_requisitions**](RequisitionsApi.md#retrieve_all_requisitions) | **GET** /api/v2/requisitions/         |

## create_requisition

> models::SpectacularRequisition create_requisition(requisition_request)

Create a new requisition

### Parameters

| Name                    | Type                                            | Description | Required   | Notes |
| ----------------------- | ----------------------------------------------- | ----------- | ---------- | ----- |
| **requisition_request** | [**RequisitionRequest**](RequisitionRequest.md) |             | [required] |

### Return type

[**models::SpectacularRequisition**](SpectacularRequisition.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_requisition_by_id

> delete_requisition_by_id(id)

Delete requisition and its end user agreement

### Parameters

| Name   | Type           | Description                                 | Required   | Notes |
| ------ | -------------- | ------------------------------------------- | ---------- | ----- |
| **id** | **uuid::Uuid** | A UUID string identifying this requisition. | [required] |

### Return type

(empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## requisition_by_id

> models::Requisition requisition_by_id(id)

Retrieve a requisition by ID

### Parameters

| Name   | Type           | Description                                 | Required   | Notes |
| ------ | -------------- | ------------------------------------------- | ---------- | ----- |
| **id** | **uuid::Uuid** | A UUID string identifying this requisition. | [required] |

### Return type

[**models::Requisition**](Requisition.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_all_requisitions

> models::PaginatedRequisitionList retrieve_all_requisitions(limit, offset)

Retrieve all requisitions belonging to the company

### Parameters

| Name       | Type            | Description                                                    | Required | Notes            |
| ---------- | --------------- | -------------------------------------------------------------- | -------- | ---------------- |
| **limit**  | Option<**i32**> | Number of results to return per page.                          |          | [default to 100] |
| **offset** | Option<**i32**> | The initial zero-based index from which to return the results. |          | [default to 0]   |

### Return type

[**models::PaginatedRequisitionList**](PaginatedRequisitionList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
