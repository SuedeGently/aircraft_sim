# Aircraft Boarding Simulator #

This is a simulator built as part of my Advanced Programming module - the logic
is written entirely in Rust, with a simple Python UI built on top of it. A
Python frontend is included, which can be run in one of two ways:

1. `make run`
2. `make; python3 ./src/main.py`

## CLI ##

Also included is a barebones command-line interface; this does not provide
access to any of the threaded mass simulation features, nor the random boarding
pattern generation, only single aircraft simulation using csv files.

```
USAGE:
    aircraft_sim <layout> <passengers>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <layout>        Layout file as csv with headers
    <passengers>    Passenger list as csv with headers
```
