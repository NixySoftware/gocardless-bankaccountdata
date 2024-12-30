# Rust OpenAPI client for GoCardless Bank Account Data

Securely access your user's bank account information for better lending, accounting, verification and financial management.

For more information, visit [https://gocardless.com/bank-account-data](https://gocardless.com/bank-account-data).

## Installation

```shell
cargo add gocardless-bankaccountdata
```

## Documentation

Documentation for is available on [Docs.rs](https://docs.rs/gocardless-bankaccountdata/latest/gocardless_bankaccountdata/).

### Documentation for API Endpoints

All URIs are relative to *https://bankaccountdata.gocardless.com*

| Class             | Method                                                                                                                                       | HTTP request                                    | Description |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- | ----------- |
| _AccountsApi_     | [**retrieve_account_balances**](docs/AccountsApi.md#retrieve_account_balances)                                                               | **GET** /api/v2/accounts/{id}/balances/         |
| _AccountsApi_     | [**retrieve_account_details**](docs/AccountsApi.md#retrieve_account_details)                                                                 | **GET** /api/v2/accounts/{id}/details/          |
| _AccountsApi_     | [**retrieve_account_metadata**](docs/AccountsApi.md#retrieve_account_metadata)                                                               | **GET** /api/v2/accounts/{id}/                  |
| _AccountsApi_     | [**retrieve_account_transactions**](docs/AccountsApi.md#retrieve_account_transactions)                                                       | **GET** /api/v2/accounts/{id}/transactions/     |
| _AgreementsApi_   | [**accept_eua**](docs/AgreementsApi.md#accept_eua)                                                                                           | **PUT** /api/v2/agreements/enduser/{id}/accept/ |
| _AgreementsApi_   | [**create_eua**](docs/AgreementsApi.md#create_eua)                                                                                           | **POST** /api/v2/agreements/enduser/            |
| _AgreementsApi_   | [**delete_eua_by_id**](docs/AgreementsApi.md#delete_eua_by_id)                                                                               | **DELETE** /api/v2/agreements/enduser/{id}/     |
| _AgreementsApi_   | [**retrieve_all_euas_for_an_end_user**](docs/AgreementsApi.md#retrieve_all_euas_for_an_end_user)                                             | **GET** /api/v2/agreements/enduser/             |
| _AgreementsApi_   | [**retrieve_eua_by_id**](docs/AgreementsApi.md#retrieve_eua_by_id)                                                                           | **GET** /api/v2/agreements/enduser/{id}/        |
| _InstitutionsApi_ | [**retrieve_all_supported_institutions_in_a_given_country**](docs/InstitutionsApi.md#retrieve_all_supported_institutions_in_a_given_country) | **GET** /api/v2/institutions/                   |
| _InstitutionsApi_ | [**retrieve_institution**](docs/InstitutionsApi.md#retrieve_institution)                                                                     | **GET** /api/v2/institutions/{id}/              |
| _RequisitionsApi_ | [**create_requisition**](docs/RequisitionsApi.md#create_requisition)                                                                         | **POST** /api/v2/requisitions/                  |
| _RequisitionsApi_ | [**delete_requisition_by_id**](docs/RequisitionsApi.md#delete_requisition_by_id)                                                             | **DELETE** /api/v2/requisitions/{id}/           |
| _RequisitionsApi_ | [**requisition_by_id**](docs/RequisitionsApi.md#requisition_by_id)                                                                           | **GET** /api/v2/requisitions/{id}/              |
| _RequisitionsApi_ | [**retrieve_all_requisitions**](docs/RequisitionsApi.md#retrieve_all_requisitions)                                                           | **GET** /api/v2/requisitions/                   |
| _TokenApi_        | [**get_a_new_access_token**](docs/TokenApi.md#get_a_new_access_token)                                                                        | **POST** /api/v2/token/refresh/                 |
| _TokenApi_        | [**obtain_new_access_slash_refresh_token_pair**](docs/TokenApi.md#obtain_new_access_slash_refresh_token_pair)                                | **POST** /api/v2/token/new/                     |

### Documentation For Models

- [Account](docs/Account.md)
- [AccountBalance](docs/AccountBalance.md)
- [AccountDetail](docs/AccountDetail.md)
- [AccountSchema](docs/AccountSchema.md)
- [AccountTransactions](docs/AccountTransactions.md)
- [BalanceAmountSchema](docs/BalanceAmountSchema.md)
- [BalanceSchema](docs/BalanceSchema.md)
- [BankTransaction](docs/BankTransaction.md)
- [CurrencyExchangeSchema](docs/CurrencyExchangeSchema.md)
- [DetailSchema](docs/DetailSchema.md)
- [EndUserAgreement](docs/EndUserAgreement.md)
- [EndUserAgreementRequest](docs/EndUserAgreementRequest.md)
- [EnduserAcceptanceDetailsRequest](docs/EnduserAcceptanceDetailsRequest.md)
- [ErrorResponse](docs/ErrorResponse.md)
- [Integration](docs/Integration.md)
- [IntegrationRetrieve](docs/IntegrationRetrieve.md)
- [JwtObtainPairRequest](docs/JwtObtainPairRequest.md)
- [JwtRefreshRequest](docs/JwtRefreshRequest.md)
- [OwnerAddressStructuredSchema](docs/OwnerAddressStructuredSchema.md)
- [PaginatedEndUserAgreementList](docs/PaginatedEndUserAgreementList.md)
- [PaginatedRequisitionList](docs/PaginatedRequisitionList.md)
- [Requisition](docs/Requisition.md)
- [RequisitionRequest](docs/RequisitionRequest.md)
- [SpectacularJwtObtain](docs/SpectacularJwtObtain.md)
- [SpectacularJwtRefresh](docs/SpectacularJwtRefresh.md)
- [SpectacularRequisition](docs/SpectacularRequisition.md)
- [StatusEnum](docs/StatusEnum.md)
- [TransactionAmountSchema](docs/TransactionAmountSchema.md)
- [TransactionSchema](docs/TransactionSchema.md)

## License

This project is available under the [MIT license](LICENSE.md).
