# Nostr LUD16 Aggregation

Want a simple script to dump all the LUD16 addresses from the metadata events from all the online relays? Just run this script. Many of the parameters are hardcoded in the file. Currently, it hits nostr.watch for all online relays. It iterates through this list and asks each relay for metadata events from the last six months. It then gets the LUD16 addresses from them and filters out any addresses that aren't from WoS. Lastly, it just appends them all to a file.

To run, just use `cargo`:

```shell
cargo run
```

The list is thrown, by default, into a file called `lud16.txt`. This can be cutomized using the `--file` flag. The default matching regex is `.+@walletofsatoshi.com`. This grabs all WoS addresses. This can be overridden using the `--matches` flag. This flag can be given multiple times to match against multiple regexes. For example, to get all the Voltage and WoS addresses and dump them to a file called `lud16_moar.txt`, run the following:

```shell
cargo run -- --file lud16_moar.txt --matches '.+@vlt.ge' --matches '.+@walletofsatoshi.com'
```

## Deduplication

This list will probably end up having alot of duplicates. To get a list of unique ones, running the following:

```shell
cat lud16.txt | sort | uniq > unique_lud16.txt
```

All the unique LUD16 addresses will be in `unique_lud16.txt`.
