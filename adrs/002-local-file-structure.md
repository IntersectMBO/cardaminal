# Local File Structure

## Context

CLI requires to keep state on the local file system. We need a well-defined folder and file-naming convention to interact with these files programmatically. 

## Decision

This tree-like structure represents the convention to be used by the CLI to store and retrieve files in the file-system of the user:

- `${ROOT_DIR}`
  - chains
    - `${CHAIN_SLUG}`
      - db
      - config.toml
  - wallets
    - `${WALLET_SLUG}`
      - state.sqlite
      - config.toml
  - transactions


The `ROOT_DIR` is the main entry point to the structure. The CLI will obtain the concrete value following this procedure:

1. if the `--root-dir` arg defined, use that value
2. if the `CARDAMINAL_ROOT_DIR` env var is defined, use that value
3. use the default value `~/.cardaminal` 


`CHAIN_SLUG` and `WALLET_SLUG` are identifiers inferred from the corresponding names entered by the user. The procedure to infer the slug is:

- take the name
- trim start/end whitespace
- replace remaining whitespace with `-`
- remove any non alphanumeric character
- fail if resulting length is 0
- fail if slug already exists in file-system