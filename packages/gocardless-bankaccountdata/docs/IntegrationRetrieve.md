# IntegrationRetrieve

## Properties

| Name                          | Type                                               | Description | Notes                     |
| ----------------------------- | -------------------------------------------------- | ----------- | ------------------------- |
| **id**                        | **String**                                         |             |
| **name**                      | **String**                                         |             |
| **bic**                       | Option<**String**>                                 |             | [optional]                |
| **transaction_total_days**    | Option<**String**>                                 |             | [optional][default to 90] |
| **max_access_valid_for_days** | Option<**String**>                                 |             | [optional]                |
| **countries**                 | **Vec<String>**                                    |             |
| **logo**                      | **String**                                         |             |
| **supported_features**        | [**Vec<serde_json::Value>**](serde_json::Value.md) |             |
| **identification_codes**      | [**Vec<serde_json::Value>**](serde_json::Value.md) |             |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
