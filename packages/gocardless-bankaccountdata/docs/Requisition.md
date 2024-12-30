# Requisition

## Properties

| Name                   | Type                                            | Description                                                              | Notes                                                                                                                        |
| ---------------------- | ----------------------------------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| **id**                 | Option<[**uuid::Uuid**](uuid::Uuid.md)>         |                                                                          | [optional][readonly]                                                                                                         |
| **created**            | Option<**String**>                              | The date & time at which the requisition was created.                    | [optional][readonly]                                                                                                         |
| **redirect**           | Option<**String**>                              | redirect URL to your application after end-user authorization with ASPSP |
| **status**             | Option<[**models::StatusEnum**](StatusEnum.md)> | status of this requisition                                               | [optional][readonly]                                                                                                         |
| **institution_id**     | **String**                                      | an Institution ID for this Requisition                                   |
| **agreement**          | Option<[**uuid::Uuid**](uuid::Uuid.md)>         | EUA associated with this requisition                                     | [optional]                                                                                                                   |
| **reference**          | Option<**String**>                              | additional ID to identify the end user                                   | [optional]                                                                                                                   |
| **accounts**           | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)>    | array of account IDs retrieved within a scope of this requisition        | [optional][readonly]                                                                                                         |
| **user_language**      | Option<**String**>                              | A two-letter country code (ISO 639-1)                                    | [optional]                                                                                                                   |
| **link**               | Option<**String**>                              | link to initiate authorization with Institution                          | [optional][readonly][default to https://ob.gocardless.com/psd2/start/3fa85f64-5717-4562-b3fc-2c963f66afa6/{$INSTITUTION_ID}] |
| **ssn**                | Option<**String**>                              | optional SSN field to verify ownership of the account                    | [optional]                                                                                                                   |
| **account_selection**  | Option<**bool**>                                | option to enable account selection view for the end user                 | [optional][default to false]                                                                                                 |
| **redirect_immediate** | Option<**bool**>                                | enable redirect back to the client after account list received           | [optional][default to false]                                                                                                 |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
