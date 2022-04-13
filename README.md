# beaconer

Periodically publishes a beacon to a broker to simulate the presence of a subscriber to a federated
topic

## Installation

```
cargo install --git https://github.com/nicolaskribas/beaconer.git
```

## Usage
```
beaconer [OPTIONS] -t <TOPIC>
```

### Options
```
-h <HOST>                   Broker's hostname [default: localhost]
    --help                  Print help information
-i <BEACON_INTERVAL>        Interval in seconds between each beacon [default: 5]
-p <PORT>                   Port number where the broker is listening on [default: 1883]
-t <TOPIC>                  Federated topic
```
