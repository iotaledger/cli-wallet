The Account Manager Interface is evaluated through the Command Line Interface of the `wallet` binary, once per
execution.

The account manager interface allows you to:
- Initialise the wallet with a mnemonic;
- Create new accounts;
- Select the account to use;
- Synchronise the accounts;

# Commands

## `./wallet`

Runs the wallet without a specified account:
- If the wallet has only one account, it will be used;
- If the wallet has more than one account, a selector will be shown to decide which account to use.

The wallet needs to be initialised (`init` command) and with at least one account (`new` command).

### Example(s)

Starts the wallet without specifying an account.
```sh
$ ./wallet
```

## `./wallet [account]`

Runs the wallet with a specified account;

The wallet needs to be initialised (`init` command).

### Example(s)

Starts the wallet with a provided account.
```sh
$ ./wallet main
```

## `./wallet help`

Displays the binary usage and exit.

### Example(s)

Display the binary usage and exit.
```sh
$ ./wallet help
```

## `./wallet init`

Initialises the wallet by creating a [stronghold](https://github.com/iotaledger/stronghold.rs) file from a provided or generated mnemonic.

The wallet can only be initialised once.

When just initialised, the wallet has no account yet, use the `new` command to create one.

### Parameters

| Name        | Optional  | Default                   |Example                                                                                                                                                                              |
| ----------- | --------- | ------------------------- |------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `mnemonic`  | ✓         | Randomly generated        | "aunt middle impose faith ramp kid olive good practice motor grab ready group episode oven matrix silver rhythm avocado assume humble tiger shiver hurt" (DO NOT USE THIS MNEMONIC) |
| `node`      | ✓         | "http://localhost:14265/" | "http://localhost:14265/"                                                                                                                                                           |

### Example(s)

Initialise the wallet with a randomly generated mnemonic and the default node.
```sh
$ ./wallet init
```

Initialise the wallet with a given mnemonic and the default node.
DO NOT USE THIS MNEMONIC.
```sh
$ ./wallet init --mnemonic "aunt middle impose faith ramp kid olive good practice motor grab ready group episode oven matrix silver rhythm avocado assume humble tiger shiver hurt"
```

Initialise the wallet with a randomly generated mnemonic and a given node.
```sh
$ ./wallet init --node "http://localhost:14265/"
```

## `./wallet new`

(init first)

### Parameters

### Example(s)

## `./wallet set-node`

### Parameters

### Example(s)

## `./wallet sync`

### Parameters

### Example(s)
