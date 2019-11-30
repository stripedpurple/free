# free
A rust implementation of the unix free command, because mac does not have one.
*Note: To be clear this is my first attempt at rust, so this might not be perfect, but it works for what I needed.*

## Build
Currently not configured to build for other platforms.

```shell script
cargo build --release
```

## Install 
First build the project then move to `/usr/local/bin/` or a more appropriate directory. 
```shell script
mv target/release/free /usr/local/bin/  
```