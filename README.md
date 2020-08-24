# motp-rs

Functions and binary tool to handle mOTP authentication in (safe) rust.

I did this because I couldn't find any solution to use my employer's VPN after
having broken my phone.

## Installation

### With rust's package manager, `cargo`

  $ cargo install motp

## CLI Usage

Here is the help:

```shell
$ motp --help
mOTP-rs 0.1
Paul O. <contact@paulollivier.fr>
mOTP tokens manipulation

USAGE:
    motp [OPTIONS] <SECRET>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --pin <PIN>    Pin code. If not given, will be expected via stdin.

ARGS:
    <SECRET>    Shared secret to use

```

So an invocation would look like this: 

```shell
$ motp 0123456789abcdef --pin 0000
137b7b
```

## Usage as a dependency

It can be used to check against a user-supplied code, provided you know it's pin and shared secret. 

cargo.toml:
```toml
[deps]
# ...other deps...
motp = "0"
```

Have a look at the docs

