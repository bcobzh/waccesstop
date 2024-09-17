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

## exemples

Find the 5 top ip in access log between 12h00 and 16h59
```sh
waccesstop  -r "2019:1[2..6]" access.log -n 5
```

Same think with awk but slower and not formated
```sh
 awk '/2019:1[2..6]/{IP[$1]++}END{for(ip in IP)print IP[ip]"\t"ip }' access.log | sort -rg | head -n5
```
## tips : compile from workstation to other server

```sh
rustup target  add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
```
