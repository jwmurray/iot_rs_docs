version 0.0.5

# Building an IOT server and IOT devices in Rust

This repo contains the Introduction to building an IOT server and IOT devices in Rust, the [IOT server in Rust][IotBook] book.
It is written and updated by the [Rust CLI working group][wg].

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

## Multi-language support
Unofficial translation:
- 中文(zh_CN)：[Rust 中的命令行应用][rust-cli-zh_CN] (2021-09-13)
- 한글(ko_KR)：[Rust 커맨드라인 애플리케이션][rust-cli-ko_KR] (2023-05-31)

[IotBook]: https://jwmurray.github.com/jwmurray/iot_rs
[wg]: https://github.com/rust-cli/meta
[rust-cli-zh_CN]: https://suibianxiedianer.github.io/rust-cli-book-zh_CN/
[rust-cli-ko_KR]: https://parksb.github.io/rust-cli-book-ko-kr/
