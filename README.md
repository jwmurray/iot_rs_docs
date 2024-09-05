version 0.0.6

# Building an IOT server and IOT devices in Rust

This repo contains the Introduction to building an IOT server and IOT devices in Rust, the [IOT server in Rust][IotBook] book.
It is written and updated by John Murray and Trevor Crawford.

## Building

Building the book requires [mdBook].
To get it:

[mdBook]: https://github.com/rust-lang/mdBook

```bash
$ cargo install mdbook
```

To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory.
To check it out, open `book/index.html` in your web browser.

To run the tests:

```bash
$ mdbook test
```

[IotBook]: https://jwmurray.github.io/iot_rs
[book-src]: https://github.com/jwmurray/iot_rs
