Ordinal Inscription Guide
=========================

Individual sats can be inscribed with arbitrary content, creating
GlobalBoost-native digital artifacts that can be held in a GlobalBoost wallet and
transferred using GlobalBoost transactions. Inscriptions are as durable, immutable,
secure, and decentralized as GlobalBoost itself.

Working with inscriptions requires a GlobalBoost full node, to give you a view of
the current state of the GlobalBoost blockchain, and a wallet that can create
inscriptions and perform sat control when constructing transactions to send
inscriptions to another wallet.

GlobalBoost Core provides both a GlobalBoost full node and wallet. However, the GlobalBoost
Core wallet cannot create inscriptions and does not perform sat control.

This requires [`ord`](https://github.com/ordinals/ord), the ordinal utility. `ord`
doesn't implement its own wallet, so `ord wallet` subcommands interact with
GlobalBoost Core wallets.

This guide covers:

1. Installing GlobalBoost Core
2. Syncing the GlobalBoost blockchain
3. Creating a GlobalBoost Core wallet
4. Using `ord wallet receive` to receive sats
5. Creating inscriptions with `ord wallet inscribe`
6. Sending inscriptions with `ord wallet send`
7. Receiving inscriptions with `ord wallet receive`
8. Batch inscribing with `ord wallet inscribe --batch`

Getting Help
------------

If you get stuck, try asking for help on the [Ordinals Discord
Server](https://discord.com/invite/87cjuz4FYg), or checking GitHub for relevant
[issues](https://github.com/ordinals/ord/issues) and
[discussions](https://github.com/ordinals/ord/discussions).

Installing GlobalBoost Core
-----------------------

GlobalBoost Core is available from [bitcoincore.org](https://bitcoincore.org/) on
the [download page](https://bitcoincore.org/en/download/).

Making inscriptions requires GlobalBoost Core 24 or newer.

This guide does not cover installing GlobalBoost Core in detail. Once GlobalBoost Core
is installed, you should be able to run `bitcoind -version` successfully from
the command line. Do *NOT* use `bitcoin-qt`.

Configuring GlobalBoost Core
------------------------

`ord` requires GlobalBoost Core's transaction index and rest interface.

To configure your GlobalBoost Core node to maintain a transaction
index, add the following to your `bitcoin.conf`:

```
txindex=1
```

Or, run `bitcoind` with `-txindex`:

```
bitcoind -txindex
```

Details on creating or modifying your `bitcoin.conf` file can be found
[here](https://github.com/bitcoin/bitcoin/blob/master/doc/bitcoin-conf.md).

Syncing the GlobalBoost Blockchain
------------------------------

To sync the chain, run:

```
bitcoind -txindex
```

…and leave it running until `getblockcount`:

```
bitcoin-cli getblockcount
```

agrees with the block count on a block explorer like [the mempool.space block
explorer](https://mempool.space/). `ord` interacts with `bitcoind`, so you
should leave `bitcoind` running in the background when you're using `ord`.

The blockchain takes about 600GB of disk space. If you have an external drive
you want to store blocks on, use the configuration option
`blocksdir=<external_drive_path>`. This is much simpler than using the
`datadir` option because the cookie file will still be in the default location
for `bitcoin-cli` and `ord` to find.

Troubleshooting
---------------

Make sure you can access `bitcoind` with `bitcoin-cli -getinfo` and that it is
fully synced.

If `bitcoin-cli -getinfo` returns `Could not connect to the server`, `bitcoind`
is not running.

Make sure `rpcuser`, `rpcpassword`, or `rpcauth` are *NOT* set in your
`bitcoin.conf` file. `ord` requires using cookie authentication. Make sure there
is a file `.cookie` in your bitcoin data directory.

If `bitcoin-cli -getinfo` returns `Could not locate RPC credentials`, then you
must specify the cookie file location.
If you are using a custom data directory (specifying the `datadir` option),
then you must specify the cookie location like
`bitcoin-cli -rpccookiefile=<your_bitcoin_datadir>/.cookie -getinfo`.
When running `ord` you must specify the cookie file location with
`--cookie-file=<your_bitcoin_datadir>/.cookie`.

Make sure you do *NOT* have `disablewallet=1` in your `bitcoin.conf` file. If
`bitcoin-cli listwallets` returns `Method not found` then the wallet is disabled
and you won't be able to use `ord`.

Make sure `txindex=1` is set. Run `bitcoin-cli getindexinfo` and it should
return something like
```json
{
  "txindex": {
    "synced": true,
    "best_block_height": 776546
  }
}
```
If it only returns `{}`, `txindex` is not set.
If it returns `"synced": false`, `globalboostd` is still creating the `txindex`.
Wait until `"synced": true` before using `ord`.

If you have `maxuploadtarget` set it can interfere with fetching blocks for
`ord` index. Either remove it or set `whitebind=127.0.0.1:8226`.

Installing `ord`
----------------

The `ord` utility is written in Rust and can be built from
[source](https://github.com/ordinals/ord). Pre-built binaries are available on the
[releases page](https://github.com/ordinals/ord/releases).

You can install the latest pre-built binary from the command line with:

```sh
curl --proto '=https' --tlsv1.2 -fsLS https://ordinals.globalboost.info/install.sh | bash -s
```

Once `ord` is installed, you should be able to run:

```
ord --version
```

Which prints out `ord`'s version number.

Creating a GlobalBoost Core Wallet
------------------------------

`ord` uses GlobalBoost Core to manage private keys, sign transactions, and
broadcast transactions to the GlobalBoost network.

To create a GlobalBoost Core wallet named `ord` for use with `ord`, run:

```
ord wallet create
```

Receiving Sats
--------------

Inscriptions are made on individual sats, using normal GlobalBoost transactions
that pay fees in sats, so your wallet will need some sats.

Get a new address from your `ord` wallet by running:

```
ord wallet receive
```

And send it some funds.

You can see pending transactions with:

```
ord wallet transactions
```

Once the transaction confirms, you should be able to see the transactions
outputs with `ord wallet outputs`.

Creating Inscription Content
----------------------------

Sats can be inscribed with any kind of content, but the `ord` wallet only
supports content types that can be displayed by the `ord` block explorer.

Additionally, inscriptions are included in transactions, so the larger the
content, the higher the fee that the inscription transaction must pay.

Inscription content is included in transaction witnesses, which receive the
witness discount. To calculate the approximate fee that an inscribe transaction
will pay, divide the content size by four and multiply by the fee rate.

Inscription transactions must be less than 400,000 weight units, or they will
not be relayed by GlobalBoost Core. One byte of inscription content costs one
weight unit. Since an inscription transaction includes not just the inscription
content, limit inscription content to less than 400,000 weight units. 390,000
weight units should be safe.

Creating Inscriptions
---------------------

To create an inscription with the contents of `FILE`, run:

```
ord wallet inscribe --fee-rate FEE_RATE --file FILE
```

Ord will output two transactions IDs, one for the commit transaction, and one
for the reveal transaction, and the inscription ID. Inscription IDs are of the
form `TXIDiN`, where `TXID` is the transaction ID of the reveal transaction,
and `N` is the index of the inscription in the reveal transaction.

The commit transaction commits to a tapscript containing the content of the
inscription, and the reveal transaction spends from that tapscript, revealing
the content on chain and inscribing it on the first sat of the input that
contains the corresponding tapscript.

Wait for the reveal transaction to be mined. You can check the status of the
commit and reveal transactions using  [the mempool.globalboost.info block
explorer](https://mempool.globalboost.info/).

Once the reveal transaction has been mined, the inscription ID should be
printed when you run:

```
ord wallet inscriptions
```

Parent-Child Inscriptions
-------------------------

Parent-child inscriptions enable what is colloquially known as collections, see
[provenance](../inscriptions/provenance.md) for more information.

To make an inscription a child of another, the parent inscription has to be
inscribed and present in the wallet. To choose a parent run `ord wallet inscriptions`
and copy the inscription id (`<PARENT_INSCRIPTION_ID>`).

Now inscribe the child inscription and specify the parent like so:

```
ord wallet inscribe --fee-rate FEE_RATE --parent <PARENT_INSCRIPTION_ID> --file CHILD_FILE
```

This relationship cannot be added retroactively, the parent has to be
present at inception of the child.

Sending Inscriptions
--------------------

Ask the recipient to generate a new address by running:

```
ord wallet receive
```

Send the inscription by running:

```
ord wallet send --fee-rate <FEE_RATE> <ADDRESS> <INSCRIPTION_ID>
```

See the pending transaction with:

```
ord wallet transactions
```

Once the send transaction confirms, the recipient can confirm receipt by
running:

```
ord wallet inscriptions
```

Receiving Inscriptions
----------------------

Generate a new receive address using:

```
ord wallet receive
```

The sender can transfer the inscription to your address using:

```
ord wallet send ADDRESS INSCRIPTION_ID
```

See the pending transaction with:
```
ord wallet transactions
```

Once the send transaction confirms, you can confirm receipt by running:

```
ord wallet inscriptions
```
