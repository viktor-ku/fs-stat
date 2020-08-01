# fs-stat

Crate has been yanked because there are better alternatives:
1. [doc.rust-lang.org/stable/std/os/unix/fs/trait.MetadataExt.html](https://doc.rust-lang.org/stable/std/os/unix/fs/trait.MetadataExt.html)
2. [nix](https://crates.io/crates/nix)

[libc](https://github.com/rust-lang/libc) `stat`, `lstat` and `fstat` bindings to rust made accessible

## Usage

Add the following to your `Cargo.toml`:

```toml
fs-stat = "0.1"
```

## Features

All functions return standard `std::io::Result` with an `std::io::Error` if it find one and
`libc::stat` if call was successful. Every unsafe bit and convertion magic from rust to C is
handled by the library, so you don't have to write it :)

## Contributing

Feel free to write an issue or suggest a pull request! I will be glad to further improve
this library!

## License

[MIT License](https://opensource.org/licenses/MIT) ([LICENSE](https://github.com/viktor-ku/fs-stat/blob/master/LICENSE))
