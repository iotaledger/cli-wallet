# Account Interface

The Account Interface is evaluated, after the Account Manager Interface, repeatedly through a prompt within the `wallet`
binary.

It is responsible for the creation and management of account addresses and their outputs, tokens, native tokens, NFTs...

## Commands

### `addresses`

Lists all account addresses.

#### Example

```sh
> Account "main": addresses
```

### `balance`

Prints the account balance.

#### Example

```sh
> Account "main": balance
```

### `burn-native-token`

Burns a native token.

#### Parameters

| Name       | Optional  | Default | Example                                                                          |
| ---------- | --------- | ------- | -------------------------------------------------------------------------------- |
| `token_id` | ✘         | N/A     | "0x08860e1f3593ba86c597cf86f61d8b04d8a714c02c7c5da7132d45be9c2ce6445c0300000000" |
| `amount`   | ✘         | N/A     | 100                                                                              |

#### Example

Burn the provided amount of a native token.
```sh
> Account "main": burn-native-token 0x08860e1f3593ba86c597cf86f61d8b04d8a714c02c7c5da7132d45be9c2ce6445c0300000000 100
```

### `burn-nft`

Burns an NFT.

#### Parameters

| Name     | Optional  | Default | Example                                                              |
| -------- | --------- | ------- | -------------------------------------------------------------------- |
| `nft_id` | ✘         | N/A     | "0x397ae8552dcf0dc604a44c9d86a5005d09f95d67e2965ea3b1c1271f9a9ae44c" |

#### Example

Burn a provided NFT.
```sh
> Account "main": burn-nft 0x397ae8552dcf0dc604a44c9d86a5005d09f95d67e2965ea3b1c1271f9a9ae44c
```

### `claim`

Tries to claim outputs with storage deposit return, expiration or timelock unlock conditions.

#### Example

```sh
> Account "main": claim
```

### `clear`

Clears the terminal.

#### Example

```sh
> Account "main": clear
```

### `consolidate`

Tries to consolidate outputs into a single one.

Note that only Basic Outputs with only an address unlock condition can be consolidated.

#### Example

```sh
> Account "main": consolidate
```

### `destroy-alias`

Destroys an alias.

#### Parameters

| Name       | Optional  | Default | Example                                                              |
| ---------- | --------- | ------- | -------------------------------------------------------------------- |
| `alias_id` | ✘         | N/A     | "0xb2bcba092bfb3fe3a12afcf21115016b27d833a7c456404fe2fe0921799f24dd" |

#### Example

```sh
> Account "main": destroy-alias 0xb2bcba092bfb3fe3a12afcf21115016b27d833a7c456404fe2fe0921799f24dd
```

### `destroy-foundry`

Destroys a foundry.

#### Parameters

| Name         | Optional  | Default | Example                                                                          |
| ------------ | --------- | ------- | -------------------------------------------------------------------------------- |
| `foundry_id` | ✘         | N/A     | "0x08b2bcba092bfb3fe3a12afcf21115016b27d833a7c456404fe2fe0921799f24dd0100000000" |

#### Example

```sh
> Account "main": destroy-foundry 0x08b2bcba092bfb3fe3a12afcf21115016b27d833a7c456404fe2fe0921799f24dd0100000000
```

### `exit`

Exits the `cli-wallet`.

#### Example

```sh
> Account "main": exit
```

### `faucet`

Requests funds from a faucet.

#### Parameters

| Name      | Optional  | Default                              | Example                                                           |
| --------- | --------- | ------------------------------------ | ----------------------------------------------------------------- |
| `url`     | ✓         | "http://localhost:8091/api/enqueue"  | "http://localhost:8091/api/enqueue"                               |
| `address` | ✓         | The latest address of the account    | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3" |

#### Examples

Request funds from a given faucet to the latest account address.
```sh
> Account "main": faucet http://localhost:8091/api/enqueue
```

Request funds from a given faucet to a given address.
```sh
> Account "main": faucet http://localhost:8091/api/enqueue rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3
```

### `help`

Displays the account interface usage.

#### Example

```sh
> Account "main": help
```

### `melt-native-token`

Melts a native token.

#### Parameters

| Name       | Optional  | Default | Example                                                                          |
| ---------- | --------- | ------- | -------------------------------------------------------------------------------- |
| `token_id` | ✘         | N/A     | "0x08860e1f3593ba86c597cf86f61d8b04d8a714c02c7c5da7132d45be9c2ce6445c0300000000" |
| `amount`   | ✘         | N/A     | 100                                                                              |

#### Example

Melt the provided amount of a native token.
```sh
> Account "main": melt-native-token 0x08860e1f3593ba86c597cf86f61d8b04d8a714c02c7c5da7132d45be9c2ce6445c0300000000 100
```

### `mint-native-token`

Mints a native token.

#### Parameters

| Name                | Optional  | Default | Example     |
| ------------------- | --------- | ------- | ----------- |
| `maximum_supply`    | ✘         | N/A     | 1000        |
| `foundry_metadata`  | ✓         | None    | "0xabcdef"  |

#### Examples

Mint a native token with a maximum supply.
```sh
> Account "main": mint-native-token 1000
```

Mint a native token with a maximum supply and foundry metadata.
```sh
> Account "main": mint-native-token 1000 0xabcdef
```

### `mint-nft`

Mints an NFT.

#### Parameters

| Name                  | Optional  | Default                           | Example                                                           |
| --------------------- | --------- | --------------------------------- | ----------------------------------------------------------------- |
| `address`             | ✓         | The first address of the account  | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3" |
| `immutable_metadata`  | ✓         | None                              | "{ key: value }"                                                  |
| `metadata`            | ✓         | None                              | "{ key: value }"                                                  |

#### Examples

Mint an NFT to the latest address of the account.
```sh
> Account "main": mint-nft
```

Mint an NFT to a given address.
```sh
> Account "main": mint-nft "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3"
```

Mint an NFT to a given address with immutable metadata and metadata.
```sh
> Account "main": mint-nft "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3" "{ key: value }" "{ key: value }"
```

### `new-address`

Generates a new address.

#### Example

```sh
> Account "main": new-address
```

### `output`

Displays an output that is stored in the account.

#### Parameters

| Name        | Optional  | Default | Example                                                                  |
| ----------- | --------- | ------- | ------------------------------------------------------------------------ |
| `output_id` | ✘         | N/A     | "0x1c7a765db0c1f5eceb0ea5578585359c5b0c1ab8d958829f5990997b93f0ec7d0100" |

#### Example

```sh
> Account "main": output 0x1c7a765db0c1f5eceb0ea5578585359c5b0c1ab8d958829f5990997b93f0ec7d0100
```

### `outputs`

Displays all outputs that are stored in the account.

#### Example

```sh
> Account "main": outputs
```

### `send`

Sends an amount to an address.

#### Parameters

| Name      | Optional  | Default | Example                                                           |
| --------- | --------- | ------- | ----------------------------------------------------------------- |
| `address` | ✘         | N/A     | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3" |
| `amount`  | ✘         | N/A     | 1000000                                                           |

#### Example

```sh
> Account "main": send rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 1000000
```

### `send-micro`

Sends a micro amount to an address with StorageDepositReturn and Expiration Unlock Conditions.

#### Parameters

| Name      | Optional  | Default | Example                                                           |
| --------- | --------- | ------- | ----------------------------------------------------------------- |
| `address` | ✘         | N/A     | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3" |
| `amount`  | ✘         | N/A     | 1                                                                 |

#### Example

```sh
> Account "main": send-micro rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 1
```

### `send-native-token`

Sends native tokens to an address with StorageDepositReturn and Expiration Unlock Condition.

#### Parameters

| Name        | Optional  | Default | Example                                                                           |
| ----------- | --------- | ------- | --------------------------------------------------------------------------------- |
| `address`   | ✘         | N/A     | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3"                 |
| `token_id`  | ✘         | N/A     | "0x08860e1f3593ba86c597cf86f61d8b04d8a714c02c7c5da7132d45be9c2ce6445c0300000000"  |
| `amount`    | ✘         | N/A     | 100                                                                               |

#### Example

```sh
> Account "main": send-native-token rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 0x08860e1f3593ba86c597cf86f61d8b04d8a714c02c7c5da7132d45be9c2ce6445c0300000000 100
```

### `send-nft`

Sends an NFT to an address.

#### Parameters

| Name      | Optional  | Default | Example                                                               |
| --------- | --------- | ------- | --------------------------------------------------------------------- |
| `address` | ✘         | N/A     | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3"     |
| `nft_id`  | ✘         | N/A     | "0x397ae8552dcf0dc604a44c9d86a5005d09f95d67e2965ea3b1c1271f9a9ae44c"  |

#### Example

```sh
> Account "main": send-nft rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 0x397ae8552dcf0dc604a44c9d86a5005d09f95d67e2965ea3b1c1271f9a9ae44c
```

### `sync`

Synchronises the account.

#### Example

```sh
> Account "main": sync
```

### `transactions`

Lists all account transactions.

#### Example

```sh
> Account "main": transactions
```

### `unspent-outputs`

Displays all unspent outputs that are stored in the account.

#### Example

```sh
> Account "main": unspent-outputs
```
