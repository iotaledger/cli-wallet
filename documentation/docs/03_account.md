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

### `exit`

Exits the `cli-wallet`.

#### Example

```sh
> Account "main": exit
```

### `faucet`

Requests funds from a faucet.

#### Parameters

| Name      | Optional  | Default                                                 | Example                                                           |
| --------- | --------- | ------------------------------------------------------- | ----------------------------------------------------------------- |
| `url`     | ✓         | "http://localhost:14265/api/plugins/faucet/v1/enqueue"  | "http://localhost:14265/api/plugins/faucet/v1/enqueue"            |
| `address` | ✓         | The latest address of the account                       | "rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3" |

#### Examples

Request funds from a given faucet to the latest account address.
```sh
> Account "main": faucet http://localhost:14265/api/plugins/faucet/v1/enqueue
```

Request funds from a given faucet to a given address.
```sh
> Account "main": faucet http://localhost:14265/api/plugins/faucet/v1/enqueue rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3
```

### `help`

Displays the account interface usage.

#### Example

```sh
> Account "main": help
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

#### Parameters

| Name    | Optional  | Default       | Example |
| ------- | --------- | ------------- | ------- |
| `` | ✓ | | |

#### Example(s)

```sh
> Account "main": mint-nft
```

### `new-address`

#### Example(s)

```sh
> Account "main": new-address
```

### `send`

#### Parameters

| Name    | Optional  | Default       | Example |
| ------- | --------- | ------------- | ------- |
| `` | ✓ | | |

#### Example(s)

```sh
> Account "main": send
```

### `send-micro`

#### Parameters

| Name    | Optional  | Default       | Example |
| ------- | --------- | ------------- | ------- |
| `` | ✓ | | |

#### Example(s)

```sh
> Account "main": send-micro
```

### `send-native-token`

#### Parameters

| Name    | Optional  | Default       | Example |
| ------- | --------- | ------------- | ------- |
| `` | ✓ | | |

#### Example(s)

```sh
> Account "main": send-native-token
```

### `send-nft`

#### Parameters

| Name    | Optional  | Default       | Example |
| ------- | --------- | ------------- | ------- |
| `` | ✓ | | |

#### Example(s)

```sh
> Account "main": send-nft
```

### `sync`

Synchronises all accounts.

#### Example

```sh
> Account "main": sync
```

### `transactions`

#### Example

```sh
> Account "main": transactions
```
