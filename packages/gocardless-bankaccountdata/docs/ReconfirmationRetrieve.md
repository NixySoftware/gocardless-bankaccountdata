# ReconfirmationRetrieve

## Properties

| Name                   | Type                                 | Description                                                                                  | Notes                |
| ---------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------- | -------------------- |
| **reconfirmation_url** | Option<**String**>                   | Reconfirmation URL to be provided to PSU.                                                    | [optional][readonly] |
| **created**            | Option<**String**>                   | Reconfirmation creation time                                                                 | [optional][readonly] |
| **url_valid_from**     | Option<**String**>                   | Datetime from when PSU will be able to access reconfirmation URL.                            | [optional][readonly] |
| **url_valid_to**       | Option<**String**>                   | Datetime until when PSU will be able to access reconfirmation URL.                           | [optional][readonly] |
| **redirect**           | Option<**String**>                   | Optional redirect URL for reconfirmation to override requisition's redirect.                 | [optional]           |
| **last_accessed**      | Option<**String**>                   | Last time when reconfirmation was accessed (this does not mean that it was accessed by PSU). | [optional][readonly] |
| **last_submitted**     | Option<**String**>                   | Last time reconfirmation was submitted (it can be submitted multiple times).                 | [optional][readonly] |
| **accounts**           | Option<[**serde_json::Value**](.md)> | Dictionary of accounts and their reconfirm and reject timestamps                             | [optional][readonly] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
