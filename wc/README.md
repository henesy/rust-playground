# wc(1)

Most of a re-implementation of the Plan 9 [wc(1)](https://9p.io/magic/man2html/1/wc) utility. 

This implementation omits the counting of broken (`-b`) UTF-8 codes. 

The `bigmac.txt` file is provided the same as the [runez2](https://github.com/henesy/runez2/) test file for verifying UTF-8 parsing accuracy. 

## Build

	cargo build

## Examples

Comparison of [plan9port](https://github.com/9fans/plan9port)'s wc(1) with this implementation: 

```
$ cargo build
   Compiling wc v0.1.0 (/home/seh/repos/rust-playground/wc)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
$ ./target/debug/wc < bigmac.txt 
lines: 10
words: 154
runes: 926
bytes: 1644
$ $PLAN9/bin/wc -lwrc bigmac.txt 
     10     154     926    1644 bigmac.txt
$ 
```

