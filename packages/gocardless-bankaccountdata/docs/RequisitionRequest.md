# RequisitionRequest

## Properties

| Name                   | Type                                    | Description                                                              | Notes                        |
| ---------------------- | --------------------------------------- | ------------------------------------------------------------------------ | ---------------------------- |
| **redirect**           | Option<**String**>                      | redirect URL to your application after end-user authorization with ASPSP |
| **institution_id**     | **String**                              | an Institution ID for this Requisition                                   |
| **agreement**          | Option<[**uuid::Uuid**](uuid::Uuid.md)> | EUA associated with this requisition                                     | [optional]                   |
| **reference**          | Option<**String**>                      | additional ID to identify the end user                                   | [optional]                   |
| **user_language**      | Option<**String**>                      | A two-letter country code (ISO 639-1)                                    | [optional]                   |
| **ssn**                | Option<**String**>                      | optional SSN field to verify ownership of the account                    | [optional]                   |
| **account_selection**  | Option<**bool**>                        | option to enable account selection view for the end user                 | [optional][default to false] |
| **redirect_immediate** | Option<**bool**>                        | enable redirect back to the client after account list received           | [optional][default to false] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
