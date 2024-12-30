# \AccountsApi

All URIs are relative to *https://bankaccountdata.gocardless.com*

| Method                                                                            | HTTP request                                | Description |
| --------------------------------------------------------------------------------- | ------------------------------------------- | ----------- |
| [**retrieve_account_balances**](AccountsApi.md#retrieve_account_balances)         | **GET** /api/v2/accounts/{id}/balances/     |
| [**retrieve_account_details**](AccountsApi.md#retrieve_account_details)           | **GET** /api/v2/accounts/{id}/details/      |
| [**retrieve_account_metadata**](AccountsApi.md#retrieve_account_metadata)         | **GET** /api/v2/accounts/{id}/              |
| [**retrieve_account_transactions**](AccountsApi.md#retrieve_account_transactions) | **GET** /api/v2/accounts/{id}/transactions/ |

## retrieve_account_balances

> models::AccountBalance retrieve_account_balances(id)

Access account balances. Balances will be returned in Berlin Group PSD2 format.

### Parameters

| Name   | Type       | Description | Required   | Notes |
| ------ | ---------- | ----------- | ---------- | ----- |
| **id** | **String** |             | [required] |

### Return type

[**models::AccountBalance**](AccountBalance.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_account_details

> models::AccountDetail retrieve_account_details(id)

Access account details. Account details will be returned in Berlin Group PSD2 format.

### Parameters

| Name   | Type       | Description | Required   | Notes |
| ------ | ---------- | ----------- | ---------- | ----- |
| **id** | **String** |             | [required] |

### Return type

[**models::AccountDetail**](AccountDetail.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_account_metadata

> models::Account retrieve_account_metadata(id)

Access account metadata. Information about the account record, such as the processing status and IBAN. Account status is recalculated based on the error count in the latest req.

### Parameters

| Name   | Type       | Description | Required   | Notes |
| ------ | ---------- | ----------- | ---------- | ----- |
| **id** | **String** |             | [required] |

### Return type

[**models::Account**](Account.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_account_transactions

> models::AccountTransactions retrieve_account_transactions(id, date_from, date_to)

Access account transactions. Transactions will be returned in Berlin Group PSD2 format.

### Parameters

| Name          | Type               | Description | Required   | Notes |
| ------------- | ------------------ | ----------- | ---------- | ----- |
| **id**        | **String**         |             | [required] |
| **date_from** | Option<**String**> |             |            |
| **date_to**   | Option<**String**> |             |            |

### Return type

[**models::AccountTransactions**](AccountTransactions.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
