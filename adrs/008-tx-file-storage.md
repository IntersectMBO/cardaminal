# File-based storage for pending transactions

## Context

We are going to store pending transactions in the "staging area" as files.

## Decision

Each chain will have it's own staging area folder, as it is feasible that one may wish to sign a transaction with multiple wallets that exist in Cardaminal, and building a transaction requires knowledge of the chain's protocol parameters.

The files will be JSON. They will have a randomly generated 4 byte ID, which will be hex encoded and used as the identifier and file name of the transaction in the staging area.

The staging area for a chain will be the `staging` folder within a chain's directory:

```sh
${CARDAMINAL_ROOT_DIR}/chains/<chain_slug>/staging/
```

### Example JSON of transaction in staging-mode
```json
{
    "version": 1,
    "created_at": <unix_ts>,
    "status": "staging",
    "inputs": [
        "<hex_tx_hash>#<tx_idx>"
    ],
    "reference_inputs?": [
        "<hex_tx_hash>#<tx_idx>"
    ],
    "outputs?": [
        {
            "address": "<bech32_address>",
            "lovelace": "<u64_amount>",
            "assets"?: {
                "<hex_policy>": {
                    "<hex_asset_name>": "<u64_amount>",
                    "<hex_asset_name>": "<u64_amount>"
                }
                "<hex_policy>": {
                    "<hex_asset_name>": "<u64_amount>",
                    "<hex_asset_name>": "<u64_amount>"
                }
            },
            "datum?": {
                "kind": ( "inline" | "hash" ),
                "bytes": "<hex_bytes>"
            }
            "script?": {
                "kind": ( "native" | "plutus_v1" | ... ),
                "bytes": "<hex_script_bytes>"
            }
        }
    ],
    "fee?": "<u64_amount>",
    "mint?": {
        "<hex_policy>": {
            "<hex_asset_name>": "<i64_amount>",
            "<hex_asset_name>": "<i64_amount>"
        }
        "<hex_policy>": {
            "<hex_asset_name>": "<i64_amount>",
            "<hex_asset_name>": "<i64_amount>"
        }
    },
    "valid_from_slot?": "<u64_amount>",
    "invalid_from_slot?": "<u64_amount>",
    "network_id?": <u32>,
    "collateral_inputs?": [
        "<hex_tx_hash>#<tx_idx>"
    ],
    "collateral_output?": {
        "address": "<bech32_address>",
        "lovelace": "<u64_amount>"
    },
    "disclosed_signers?": [
        "hex_pub_key_hash"
    ],
    "scripts?": [
        {
            "kind": ( "native" | "plutus_v1" | ... ),
            "bytes": "<hex_script_bytes>"
        }
    ],
    "datums?": [
        "<hex_script_bytes>"
    ],
    "redeemers?": {
        "spend:<hex_tx_hash>#<tx_idx>": {
            "ex_units?": {
                "mem": <u64_amount>,
                "steps": <u64_amount>
            }
        },
        "mint:<hex_policy_id>": {
            ...
        },
        "reward:<hex_script_hash>": {
            ...
        },
        "cert:<cert_index>": {
            ...
        }
    }
    "signature_amount_override?": <u8_amount>,
    "change_address?": "<bech32_address>"
}
```

### Example JSON of transaction in signing-mode
```json
{
    "version": 1,
    "created_at": <unix_ts>,
    "status": "signing",
    "tx_hash": "<hex_tx_hash>",
    "tx_body": "<hex_tx_body_bytes>",
    "signatures": {
        "<hex_public_key>": "<hex_signature_bytes>",
    }
}
```