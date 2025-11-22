# Turing machine simulator
CLI Turing machine simulator made completely on rust. Designed witht the idea on mind of being simple to use, fast and tiny.

This program is able to simulate a multitape or singletape deterministic Turing machine.

## Installation
```bash
git clone git@github.com:TheLazyFerret/turing-machine-simulator.git
cd turing-machine-simulator
cargo install .
```

## Usage
```
Usage: turing-machine-simulator [OPTIONS] --turing-path <TURING_PATH> <STRING>

Arguments:
  <STRING>  String to be tested on the Turing machine

Options:
  -t, --turing-path <TURING_PATH>  Path where the Turing machine configuration is located
  -s, --shell...                   Treat failures as errors
  -h, --help                       Print help
  -V, --version                    Print version
```

## Turing machine configuration
They are defined in a `.toml` file.
```
ntapes = <integer>         # Number of tapes on the machine (minimum one).
initial = <integer>        # The initial state.
accept = <array<integer>>  # Acceptance states.
```
Then, each transitition is defined inside an array of tables.
```
[[transitition]]
from = <integer>            # The arrival state.
next = <integer>            # The destiny state.
read = <string>             # The character read in each tape.
write = <string>            # The character write in each tape.
direction = <string>        # The direction in each tape (L|R|S).
```
In [example](example/) you can find some configurations, for both singletape and multitape.

## License
This project is under the [MIT](LICENSE) license.