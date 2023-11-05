# Chain Configuration

## Context

Our CLI requires certain configuration data about each chain it connects to. We need to define whats the minimum set of data and how to store it.

## Decision

We anticipate that the following chain-specific data will be required to fulfill the scope of the CLI:

- version: an identifier of the config schema version for backward compatibility
- name: a descriptive name given by the user for later identification.
- upstream: the network address of a trusted node that can be used to interact with the network via n2n protocols.
- magic: the network magic of the chain required for the handshaking procedure.
- after: slot / hash of the point in the chain where the sync process should start from

We'll be storing the config data on the filesystem of the user. Each chain will have it's own directory under the user's home path.

For example, for a chain named `mainnet`, we'll store it's config in:

```sh
${CARDAMINAL_ROOT_DIR}/chains/mainnet/config.toml
```

The content of the config file will use `TOML` format and the schema is to be inferred from the following example:

```toml
version = "v1alpha"
name = "mainnet"
magic = 11111
created_at = "2023-03-03"

[upstream]
address = "my-relay:33301"

[after]
slot = 111
hash = "abcdefabcdef"
```
