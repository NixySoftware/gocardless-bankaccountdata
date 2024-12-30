# DetailSchema

## Properties

| Name                           | Type                                                                                | Description                                                                                    | Notes      |
| ------------------------------ | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ---------- |
| **resource_id**                | Option<**String**>                                                                  | resourceId                                                                                     | [optional] |
| **iban**                       | Option<**String**>                                                                  | iban                                                                                           | [optional] |
| **bban**                       | Option<**String**>                                                                  | bban                                                                                           | [optional] |
| **scan**                       | Option<**String**>                                                                  | SortCodeAccountNumber returned by some UK banks (6 digit Sort Code and 8 digit Account Number) | [optional] |
| **msisdn**                     | Option<**String**>                                                                  | msisdn                                                                                         | [optional] |
| **currency**                   | Option<**String**>                                                                  | currency                                                                                       | [optional] |
| **owner_name**                 | Option<**String**>                                                                  | ownerName                                                                                      | [optional] |
| **name**                       | Option<**String**>                                                                  | name                                                                                           | [optional] |
| **display_name**               | Option<**String**>                                                                  | displayName                                                                                    | [optional] |
| **product**                    | Option<**String**>                                                                  | product                                                                                        | [optional] |
| **cash_account_type**          | Option<**String**>                                                                  | cashAccountType                                                                                | [optional] |
| **status**                     | Option<**String**>                                                                  | status                                                                                         | [optional] |
| **bic**                        | Option<**String**>                                                                  | bic                                                                                            | [optional] |
| **linked_accounts**            | Option<**String**>                                                                  | linkedAccounts                                                                                 | [optional] |
| **masked_pan**                 | Option<**String**>                                                                  | maskedPan                                                                                      | [optional] |
| **usage**                      | Option<**String**>                                                                  | usage                                                                                          | [optional] |
| **details**                    | Option<**String**>                                                                  | details                                                                                        | [optional] |
| **owner_address_unstructured** | Option<**Vec<String>**>                                                             | ownerAddressUnstructured                                                                       | [optional] |
| **owner_address_structured**   | Option<[**models::OwnerAddressStructuredSchema**](OwnerAddressStructuredSchema.md)> | ownerAddressStructured                                                                         | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
