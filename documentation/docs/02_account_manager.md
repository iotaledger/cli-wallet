The Account Manager Interface is evaluated through the Command Line Interface of the `wallet` binary, once per
execution.

The account manager interface allows you to:
- Initialise the wallet with a mnemonic;
- Create new accounts;
- Select the account to use;
- Synchronise the accounts;

# Commands

## `help`

Displays the binary usage and exit.

### Parameters

No parameters.

### Example(s)

Display the binary usage and exit.
```sh
./wallet help
```

## `init`

The wallet can only be initialised once.

### Parameters

| Name        | Optional  | Default                   |Example                                                                                                                                                   |
| ----------- | --------- | ------------------------- |--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mnemonic`  | ✓         | Randomly generated        | "aunt middle impose faith ramp kid olive good practice motor grab ready group episode oven matrix silver rhythm avocado assume humble tiger shiver hurt"  |
| `node`      | ✓         | "http://localhost:14265/" | "http://localhost:14265/"                                                                                                                                 |

### Example(s)

Initialise the wallet with a randomly generated mnemonic and the default node.
```sh
./wallet init
```

Initialise the wallet with a given mnemonic and the default node.
```sh
./wallet init --mnemonic "aunt middle impose faith ramp kid olive good practice motor grab ready group episode oven matrix silver rhythm avocado assume humble tiger shiver hurt"
```

Initialise the wallet with a a randomly generated mnemonic and a given node.
```sh
./wallet init --node "http://localhost:14265/"
```

## `new`

(init first)

### Parameters

### Example(s)

## `select`

### Parameters

### Example(s)

## `set-node`

### Parameters

### Example(s)

## `sync`

### Parameters

### Example(s)
