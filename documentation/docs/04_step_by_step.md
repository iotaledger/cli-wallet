# Step by step examples

In these step by step examples, we present how to create a wallet and do some of the most common use cases.

It is advised to do them all at least once in the given order to understand the workflow.

## Setup

Initialise the wallet with a given node and a randomly generated mnemonic.
```sh
$ ./wallet init --node [NODE API URL]
> ...
> INFO  Mnemonic stored successfully
```

Create a main account.
```sh
$ ./wallet new main
> ...
> INFO  Created account "main"
> Account "main": exit
```

Create a savings account.
```sh
$ ./wallet new savings
> ...
> INFO  Created account "savings"
> Account "savings": exit
```

## Tokens

Get some funds from the faucet to the main account.
```sh
$ ./wallet main
> Account "main": faucet [FAUCET ENQUEUE API URL]
> ...
> Account "main": sync
> ...
> INFO  Synced: AccountBalance ...
> Account "main": exit
```

### Send a regular amount

Get an address from the savings account.
```sh
$ ./wallet savings
> Account "savings": addresses
> INFO  Address 0: [ADDRESS]
> Account "savings": exit
```

Send a regular amount from the main account to the savings address.
```sh
$ ./wallet main
> Account "main": send [ADDRESS] 1000000
> ...
> INFO  Transaction sent:
> tx id: 0x076f3e1b9c18b00037308a338a423f3dad7c3ab08a2dc38cc847154fa86e1709
> Some(BlockId(0x56efae2c8e8b66e7c98e45b5886f889aefc518903ac9af342771195912fd72ee))
> Account "main": exit
```

### Send a micro amount

Generate a new address from the savings account.
```sh
$ ./wallet savings
> Account "savings": new-address
> ...
> INFO  Address 1: [ADDRESS]
> Account "savings": exit
```

Send a micro amount from the main account to the savings address.
```sh
$ ./wallet main
> Account "main": send-micro [ADDRESS] 1
> ...
> INFO  Micro transaction sent:
> tx id: 0xcb7cd9732237ae679b82b1a09ef5d23f62e11142ea066d88c50e77962655889c
> Some(BlockId(0xc1b5bf265160aaba2b7e26208a467c6582e71c005235458b04649d407aa79c55))
> Account "main": exit
```

Check the savings balance.
```sh
$ ./wallet savings
> Account "savings": balance
> ...
> INFO  AccountBalance ...
> Account "savings": exit
```

## Native tokens

### Mint

Mint native tokens, with foundry metadata, from the main account.
```sh
$ ./wallet main
> Account "main": mint-native-token 1000 1000 --foundry-metadata-hex 0xabcdef
> ...
> INFO  Native token minting transaction sent:
> tx id: 0x42885ee7511aede64fce578306e13515b69b60237f71d4124f51cc0a7c963c64
> Some(BlockId(0x15c5f2fb4670191d201e030221b365ec6fe8be52ee5a8a3bc34f11dce46cc232))
> Account "main": exit
```

### Send

Generate a new address from the savings account.
```sh
$ ./wallet savings
> Account "savings": new-address
> ...
> INFO  Address 2: [ADDRESS]
> Account "savings": exit
```

Send native tokens to the savings address.
```sh
$ ./wallet main
> Account "main": sync
> ...
> INFO  Synced: AccountBalance ...TokenId([TOKEN ID])...
> Account "main": send-native-token [ADDRESS] [TOKEN ID] 100
> INFO  Native token transaction sent:
> tx id: 0xb02d6a85c2108c68bbbbd3b3070420ca617f384bf22f6e8a3d6f6414a4bfafef
> Some(BlockId(0x5c10008ee5979134557a8ad785c5bdf946beb8bfd3d1f032fee549a58cc0ca47))
> Account "main": exit
```

## NFTs

### Mint

Mint an NFT.
```sh
$ ./wallet main
> Account "main": mint-nft
> ...
> INFO  NFT minting transaction sent:
> tx id: 0x7926e95119549685e2cf47fc1be2e935d22e8412fe451e6cdb5373a0f28bff43
> Some(BlockId(0x228e34699faaa7d6ccb0d00716f081aa451efb786a9b98d5c14d7bef4ab4e244))
> Account "main": exit
```

### Send

Generate a new address from the savings account.
```sh
$ ./wallet savings
> Account "savings": new-address
> ...
> INFO  Address 3: [ADDRESS]
> Account "savings": exit
```

Send the NFT to the savings address.
```sh
$ ./wallet main
> Account "main": sync
> ...
> INFO  Synced: AccountBalance ...NftId([NFT ID])...
> Account "main": send-nft [ADDRESS] [NFT ID]
> INFO  Nft transaction sent:
> tx id: 0x1f2da6d3d3d57510ed701301f0625e4e71d1ee5a20050a9406ea1ae681650462
> Some(BlockId(0xffd725d4bb96897dd34515b856279e250cfa79e2300841f7a11987a14a4daf2f))
> Account "main": exit
```

## Transactions

List the transactions of the main account.
```sh
$ ./wallet main
> Account "main": transactions
> ...
> INFO  Transaction...
> Account "main": exit
```
