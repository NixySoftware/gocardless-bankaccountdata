# \InstitutionsApi

All URIs are relative to *https://bankaccountdata.gocardless.com*

| Method                                                                                                                                  | HTTP request                       | Description |
| --------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- | ----------- |
| [**retrieve_all_supported_institutions_in_a_given_country**](InstitutionsApi.md#retrieve_all_supported_institutions_in_a_given_country) | **GET** /api/v2/institutions/      |
| [**retrieve_institution**](InstitutionsApi.md#retrieve_institution)                                                                     | **GET** /api/v2/institutions/{id}/ |

## retrieve_all_supported_institutions_in_a_given_country

> Vec<models::Integration> retrieve_all_supported_institutions_in_a_given_country(access_scopes_supported, account_selection_supported, business_accounts_supported, card_accounts_supported, corporate_accounts_supported, country, pending_transactions_supported, private_accounts_supported, read_debtor_account_supported, read_refund_account_supported, separate_continuous_history_consent_supported, ssn_verification_supported)

List all available institutions

### Parameters

| Name                                              | Type               | Description                                                                       | Required | Notes |
| ------------------------------------------------- | ------------------ | --------------------------------------------------------------------------------- | -------- | ----- |
| **access_scopes_supported**                       | Option<**String**> | Boolean value, indicating if access scopes are supported                          |          |
| **account_selection_supported**                   | Option<**String**> | Boolean value, indicating if account selection is supported                       |          |
| **business_accounts_supported**                   | Option<**String**> | Boolean value, indicating if business accounts are supported                      |          |
| **card_accounts_supported**                       | Option<**String**> | Boolean value, indicating if card accounts are supported                          |          |
| **corporate_accounts_supported**                  | Option<**String**> | Boolean value, indicating if corporate accounts are supported                     |          |
| **country**                                       | Option<**String**> | ISO 3166 two-character country code                                               |          |
| **pending_transactions_supported**                | Option<**String**> | Boolean value, indicating if pending transactions are supported                   |          |
| **private_accounts_supported**                    | Option<**String**> | Boolean value, indicating if private accounts are supported                       |          |
| **read_debtor_account_supported**                 | Option<**String**> | Boolean value, indicating if debtor account can be read before submitting payment |          |
| **read_refund_account_supported**                 | Option<**String**> | Boolean value, indicating if read refund account is supported                     |          |
| **separate_continuous_history_consent_supported** | Option<**String**> | Boolean value, indicating if separate consent for continuous history is supported |          |
| **ssn_verification_supported**                    | Option<**String**> | Boolean value, indicating if ssn verification is supported                        |          |

### Return type

[**Vec<models::Integration>**](Integration.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_institution

> models::IntegrationRetrieve retrieve_institution(id)

Get details about a specific Institution and its supported features

### Parameters

| Name   | Type       | Description | Required   | Notes |
| ------ | ---------- | ----------- | ---------- | ----- |
| **id** | **String** |             | [required] |

### Return type

[**models::IntegrationRetrieve**](IntegrationRetrieve.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
