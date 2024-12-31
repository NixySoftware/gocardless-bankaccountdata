# EndUserAgreement

## Properties

| Name                      | Type                                                       | Description                                                                                     | Notes                                                       |
| ------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| **id**                    | Option<[**uuid::Uuid**](uuid::Uuid.md)>                    | The ID of this End User Agreement, used to refer to this end user agreement in other API calls. | [optional][readonly]                                        |
| **created**               | Option<**String**>                                         | The date & time at which the end user agreement was created.                                    | [optional][readonly]                                        |
| **institution_id**        | **String**                                                 | an Institution ID for this EUA                                                                  |
| **max_historical_days**   | Option<**i16**>                                            | Maximum number of days of transaction data to retrieve.                                         | [optional][default to 90]                                   |
| **access_valid_for_days** | Option<**i16**>                                            | Number of days from acceptance that the access can be used.                                     | [optional][default to 90]                                   |
| **access_scope**          | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array containing one or several values of ['balances', 'details', 'transactions']               | [optional]default to ["balances","details","transactions"]] |
| **accepted**              | Option<**String**>                                         | The date & time at which the end user accepted the agreement.                                   | [optional][readonly]                                        |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
