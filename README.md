# rust-stats-block


<p align="center">
  <img width="100%" src="https://cdn.jsdelivr.net/gh/LiQingMuBai/rust-stats-block/blocks.gif">
</p>

#### Building for source
For build the binary just:
```sh
$ cargo build --release --verbose
```
To run as debug, just run this example:
```sh
$ cargo run -- <start block number> <end block number> <node's url>
```
### Installation
Install simple typing:

```rustdingtalk
cargo install rstats-blocks
```

### Documentation
The documentation, for now, is the help return of tool:

```sh

USAGE:
    rstats-blocks -- [_start]  [_end]  [_node]


ARGS:
    <_start>    	               from start block number
    <_end>                       to end block number
    <_node>                      test ethereum blockchain's node url
```
