# Setup

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

# Send an amount

Get some funds from the faucet.
<!-- TODO: replace with testnet URL -->
```sh
$ ./wallet main
> Account "main" command (h for help): faucet [URL]
> ...
> Account "main" command (h for help): sync
> INFO  Synced: AccountBalance ...
> CTRL-C
```

```sh
$ ./wallet savings
> Account "savings" command (h for help): list-addresses
> INFO  Address 0: [ADDR]
> CTRL-C
```

```sh
$ ./wallet main
> Account "main" command (h for help): send [ADDR] 500000
> INFO  Transaction created ...
```
