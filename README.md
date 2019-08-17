# rot13

Rot-13 encryption and decryption (written in rust)

[![Build Status](https://travis-ci.com/cameronp98/rot13-rs.svg?branch=master)](https://travis-ci.com/cameronp98/rot13-rs)

```
USAGE:
    rot13 [FLAGS] [OPTIONS] --decrypt --encrypt

FLAGS:
    -d, --decrypt    decrypt the provided input
    -e, --encrypt    encrypt the provided input
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <PATH>     the input file to use. defaults to stdin
    -o, --output <PATH>    the output file to use. defaults to stdout
```

Library usage:

```rust
use rot13::{rot13_slice, Mode};

fn main() {
    let input = b"Hello, World!";
    
    // try encryption
    let encrypted = rot13_slice(Mode::Encrypt, input);
    println!("{}", ::std::str::from_utf8(&encrypted).unwrap());
    
    // since case is preserved during encryption, the decrypted
    // product should be the same as the original input
    let decrypted = rot13_slice(Mode::Decrypt, &encrypted);
    assert_eq!(input, decrypted.as_slice());
}
```