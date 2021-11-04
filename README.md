# b45
## A Base45 encoder and decoder written in Rust
It works similarly to the base45 crate but decoding may be faster. The algorithm has been tested against the examples from the Base45 draft:
https://datatracker.ietf.org/doc/draft-faltstrom-base45/

## Usage
For encoding a string:
```rust,no_run
    let str_encoded = b45::encode("...");
```
And for decoding a string:
```rust,no_run
    let str_decoded = b45::decode("QED8WEX0");
```
