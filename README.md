# IOTA Stardust CLI Wallet

Command line interface application for the [IOTA wallet library](https://github.com/iotaledger/wallet.rs).

## Usage

After downloading the CLI, initialize the signer for the wallet. On Mac and Linux you will first need to `chmod +x ./wallet`.

```
./wallet init --node http://node.url:port --mnemonic MNEMONIC
// Example:
./wallet init --node "http://localhost:14265" --mnemonic "giant dynamic museum toddler six deny defense ostrich bomb access mercy 
blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally"
```

Then create a new account

```
./wallet new ALIAS
// Example:
./wallet new Alice
```

If you already created an account, you can just run the CLI without args to get to the account selector:

```
./wallet
```

Alternatively, you can select an existing account by it's alias, to use with the `account` command:

```
./wallet account Alice
```

## Commands

The wallet CLI has a set of main commands accesible with `./wallet COMMAND [ARGS]` and a dedicated command list for the account prompt.

### Main commands

#### help [COMMAND]

Prints the CLI help information. If a command is specified, the command's help will be printed.

#### init [MNEMONIC]

Initialize the wallet with a mnemonic, if none is provided, a new one will be generated.

#### new [ALIAS]

Creates a new account, optionally takes an account alias.

#### account ALIAS

Selects the account associated with the specified alias.

#### set-node NODE

Set the node url to be used. 
```
./wallet set-node "http://localhost:14265"
```

#### sync

Synchronizes all accounts with the Tangle.


### Account prompt commands

#### help [COMMAND]

Prints the CLI help information. If a command is specified, the command's help will be printed.

#### exit

Exits the account prompt.

#### sync

Synchronizes the account with the Tangle and returns the balance.

#### address

Generates a new unused address.

#### balance

Gets the account balance.

#### list-addresses

Lists the account's addresses.

#### list-transactions

Lists the account's transactions.

#### send ADDRESS AMOUNT

Send funds from the account to the given Bech32 address.
Example: `send atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r 1000000`

#### send-native ADDRESS TOKEN_ID AMOUNT

Send native tokens to a bech32 address: `send-native atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r 08e3a2f76cc934bc0cc21575b4610c1d7d4eb589ae0100000000000000000000000000000000 10`

## Caveats

### Database path

By default the database path is `./wallet-cli-database` but you can change this with the `WALLET_DATABASE_PATH` environment variable:

```
export WALLET_DATABASE_PATH=/path/to/database # or add it to your .bashrc, .zshrc
./wallet [COMMAND] [OPTIONS]
```

## Contributing

To run the CLI from source, install Rust (usually through [Rustup](https://rustup.rs/)) and run the following commands:

```
git clone https://github.com/iotaledger/cli-wallet
cargo run -- [COMMAND] [OPTIONS]
```
