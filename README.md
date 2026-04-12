# neocom

EVE Online CLI toolkit for capsuleers. Market prices, route safety, pilot intel, and more.

## Installation

```bash
# From source
cargo install --path .

# Or build manually
cargo build --release
./target/release/neocom
```

## Commands

### `neocom travel <origin> <destination>`

Fetch route via ESI and show danger rating per hop.

```bash
neocom travel Jita Amarr
neocom travel Jita Amarr --hours 48
neocom travel Jita Amarr --route safest
```

### `neocom price <item> [quantity]`

Fetch live market prices from Jita.

```bash
neocom price "Fullerite-C50"
neocom price "Tritanium" 10000

# Batch from TSV file
neocom price --file items.tsv
```

### `neocom intel <pilot>`

Pilot intel and kill history from zKillboard.

```bash
neocom intel "Angry Gankerson"
```

### `neocom system <name>`

System overview with kill activity.

```bash
neocom system Jita
neocom system Rancer
```

### `neocom wh <class>`

Wormhole sites for a class.

```bash
neocom wh c3
neocom wh c5
```

### `neocom status`

EVE server status.

```bash
neocom status
```

## Data Sources

- [ESI](https://esi.evetech.net) - Routes, market data, system info
- [zKillboard](https://zkillboard.com) - Kill data, pilot history

## License

MIT
