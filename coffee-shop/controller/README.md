# dephy-gacha-controller

## Prepare

In this example, you should have a [dephy messaging network](https://github.com/dephy-io/dephy-messaging-network-self-hosted/tree/main/dephy-messaging-network) running on `ws://127.0.0.1:8000`.
You can modify `--nostr-relay` if it is running on a different address.

## Run Controller (this is on gacha equipment side)

The controller daemon need your solana keypair.

```shell
mkdir data
# copy your keypair to data/solana-keypair
```

```shell
cargo run --bin dephy-gacha-controller -- --nostr-relay ws://127.0.0.1:8000 --machine-pubkeys 91550af28891a6ac6c73e0d415ed5ee9ea5603ef6d276df623a8b80254519ab2 --admin-pubkey <the pubkey of your private nostr key or use random one> --solana-rpc-url https://api.devnet.solana.com
```
