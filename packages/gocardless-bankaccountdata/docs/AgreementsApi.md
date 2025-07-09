# \AgreementsApi

All URIs are relative to *https://bankaccountdata.gocardless.com*

| Method                                                                          | HTTP request                                        | Description |
| ------------------------------------------------------------------------------- | --------------------------------------------------- | ----------- |
| [**accept_eua**](AgreementsApi.md#accept_eua)                                   | **PUT** /api/v2/agreements/enduser/{id}/accept/     |
| [**create_eua**](AgreementsApi.md#create_eua)                                   | **POST** /api/v2/agreements/enduser/                |
| [**create_eua_reconfirmation**](AgreementsApi.md#create_eua_reconfirmation)     | **POST** /api/v2/agreements/enduser/{id}/reconfirm/ |
| [**delete_eua_by_id**](AgreementsApi.md#delete_eua_by_id)                       | **DELETE** /api/v2/agreements/enduser/{id}/         |
| [**retrieve_all_agreements**](AgreementsApi.md#retrieve_all_agreements)         | **GET** /api/v2/agreements/enduser/                 |
| [**retrieve_eua_by_id**](AgreementsApi.md#retrieve_eua_by_id)                   | **GET** /api/v2/agreements/enduser/{id}/            |
| [**retrieve_eua_reconfirmation**](AgreementsApi.md#retrieve_eua_reconfirmation) | **GET** /api/v2/agreements/enduser/{id}/reconfirm/  |

## accept_eua

> models::EndUserAgreement accept_eua(id, enduser_acceptance_details_request)

Accept an end-user agreement via the API

### Parameters

| Name                                   | Type                                                                      | Description                                        | Required   | Notes |
| -------------------------------------- | ------------------------------------------------------------------------- | -------------------------------------------------- | ---------- | ----- |
| **id**                                 | **uuid::Uuid**                                                            | A UUID string identifying this end user agreement. | [required] |
| **enduser_acceptance_details_request** | [**EnduserAcceptanceDetailsRequest**](EnduserAcceptanceDetailsRequest.md) |                                                    | [required] |

### Return type

[**models::EndUserAgreement**](EndUserAgreement.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_eua

> models::EndUserAgreement create_eua(end_user_agreement_request)

API endpoints related to end-user agreements.

### Parameters

| Name                           | Type                                                      | Description | Required   | Notes |
| ------------------------------ | --------------------------------------------------------- | ----------- | ---------- | ----- |
| **end_user_agreement_request** | [**EndUserAgreementRequest**](EndUserAgreementRequest.md) |             | [required] |

### Return type

[**models::EndUserAgreement**](EndUserAgreement.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_eua_reconfirmation

> models::ReconfirmationRetrieve create_eua_reconfirmation(id, reconfirmation_retrieve_request)

Create EUA reconfirmation

### Parameters

| Name                                | Type                                                                          | Description                                        | Required   | Notes |
| ----------------------------------- | ----------------------------------------------------------------------------- | -------------------------------------------------- | ---------- | ----- |
| **id**                              | **uuid::Uuid**                                                                | A UUID string identifying this end user agreement. | [required] |
| **reconfirmation_retrieve_request** | Option<[**ReconfirmationRetrieveRequest**](ReconfirmationRetrieveRequest.md)> |                                                    |            |

### Return type

[**models::ReconfirmationRetrieve**](ReconfirmationRetrieve.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_eua_by_id

> models::SuccessfulDeleteResponse delete_eua_by_id(id)

Delete an end user agreement

### Parameters

| Name   | Type           | Description                                        | Required   | Notes |
| ------ | -------------- | -------------------------------------------------- | ---------- | ----- |
| **id** | **uuid::Uuid** | A UUID string identifying this end user agreement. | [required] |

### Return type

[**models::SuccessfulDeleteResponse**](SuccessfulDeleteResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_all_agreements

> models::PaginatedEndUserAgreementList retrieve_all_agreements(limit, offset)

Retrieve all End User Agreements belonging to the company

### Parameters

| Name       | Type            | Description                                                    | Required | Notes            |
| ---------- | --------------- | -------------------------------------------------------------- | -------- | ---------------- |
| **limit**  | Option<**i32**> | Number of results to return per page.                          |          | [default to 100] |
| **offset** | Option<**i32**> | The initial zero-based index from which to return the results. |          | [default to 0]   |

### Return type

[**models::PaginatedEndUserAgreementList**](PaginatedEndUserAgreementList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_eua_by_id

> models::EndUserAgreement retrieve_eua_by_id(id)

Retrieve end user agreement by ID

### Parameters

| Name   | Type           | Description                                        | Required   | Notes |
| ------ | -------------- | -------------------------------------------------- | ---------- | ----- |
| **id** | **uuid::Uuid** | A UUID string identifying this end user agreement. | [required] |

### Return type

[**models::EndUserAgreement**](EndUserAgreement.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_eua_reconfirmation

> models::ReconfirmationRetrieve retrieve_eua_reconfirmation(id)

Retrieve EUA reconfirmation

### Parameters

| Name   | Type           | Description                                        | Required   | Notes |
| ------ | -------------- | -------------------------------------------------- | ---------- | ----- |
| **id** | **uuid::Uuid** | A UUID string identifying this end user agreement. | [required] |

### Return type

[**models::ReconfirmationRetrieve**](ReconfirmationRetrieve.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
