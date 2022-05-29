# Step by step examples

In these step by step examples, we present how to create a wallet and do some of the most common use cases.

## Setup

Initialise the wallet with a given node and a randomly generated mnemonic.
<!-- TODO: replace with testnet URL -->
```sh
$ ./wallet init --node [URL]
> ...
> INFO  Mnemonic stored successfully
```

Create a main account.
```sh
$ ./wallet new main
> ...
> INFO  Created account "main"
> CTRL-C
```

Create a savings account.
```sh
$ ./wallet new savings
> ...
> INFO  Created account "savings"
> CTRL-C
```

## Send an amount

Get some funds from the faucet to the main account.
<!-- TODO: replace with testnet URL -->
```sh
$ ./wallet main
> Account "main" command (h for help): faucet [URL]
> ...
> Account "main" command (h for help): sync
> ...
> INFO  Synced: AccountBalance ...
> CTRL-C
```

### Regular amount

Get an address from the savings account.
```sh
$ ./wallet savings
> Account "savings" command (h for help): addresses
> INFO  Address 0: [ADDR]
> CTRL-C
```

Send a regular amount from the main account to the savings address.
```sh
$ ./wallet main
> Account "main" command (h for help): send [ADDR] 1000000
> ...
> INFO  Transaction created ...
> CTRL-C
```

### Micro amount

Generate a new address from the savings account.
```sh
$ ./wallet savings
> Account "savings" command (h for help): new-address
> ...
> INFO  Address 1: [ADDR]
> CTRL-C
```

Send a micro amount from the main account to the savings address.
```sh
$ ./wallet main
> Account "main" command (h for help): send-micro [ADDR] 1
> ...
> INFO  Transaction created ...
> CTRL-C
```

Check the savings balance.
```sh
$ ./wallet savings
> Account "savings" command (h for help): balance
> ...
> INFO  AccountBalance ...
> CTRL-C
```
