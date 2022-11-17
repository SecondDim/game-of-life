# game-of-life

Just for practicing Rust

## usage

1. use default.ini

- simple run use `cargo run`
- build with `cargo build` or `cargo build --release`
- use binary file: `./target/debug/gol` or `./target/release/gol`

2. use custom file

- `cargo run path/to/init/file`
- `./target/debug/gol path/to/init/file` or `./target/release/gol path/to/init/file`


## .ini args

```
# Init world size => width hight
world 110 55

# Add seed => x y
Block 5 0
Beehive 5 5
loaf 5 10
boat 5 15
tub 5 20

blinker 10 10
toad 10 15
beacon 10 20
pulsar 10 25

penta_decathlon 20 10
glider 20 20
lightweightspaceship 20 30
mwss 20 40
HWSS 20 50

# Add line => x y len
vertical_line 50 50 999
horizontal 50 50 999
```
