# waccesstop

Little project to test rust language on apache/nginx access log file  
I'm just starting rust  .... 


## usage 

```sh
./target/release/waccesstop -h 
waccesstop 0.1.0
top ip in apache/nginx access logs

USAGE:
    waccesstop [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n <lines>             number ip to display, default 10
    -r, --regex <regex>    regex to filter lines

ARGS:
    <INPUT>    the access log file, could be gz
``` 


## tips : compile from fedora to other gnu/linux

```sh
cargo build --release --target=x86_64-unknown-linux-musl
```
