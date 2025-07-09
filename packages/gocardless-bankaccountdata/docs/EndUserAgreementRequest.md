# EndUserAgreementRequest

## Properties

| Name                      | Type                                                       | Description                                                                       | Notes                                                       |
| ------------------------- | ---------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| **institution_id**        | **String**                                                 | an Institution ID for this EUA                                                    |
| **max_historical_days**   | Option<**i16**>                                            | Maximum number of days of transaction data to retrieve.                           | [optional][default to 90]                                   |
| **access_valid_for_days** | Option<**i16**>                                            | Number of days from acceptance that the access can be used.                       | [optional][default to 90]                                   |
| **access_scope**          | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array containing one or several values of ['balances', 'details', 'transactions'] | [optional]default to ["balances","details","transactions"]] |
| **reconfirmation**        | Option<**bool**>                                           | if this agreement can be extended. Supported by GB banks only.                    | [optional][default to false]                                |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
