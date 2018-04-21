# 'The Rust Programming Language' as EBook

This repository contains stuff to convert [this book](http://doc.rust-lang.org/book/) to EPUB.

## DIY

Install:

- pandoc
- Rust and cargo

Then run:

```sh
$ cargo run --release
```

## Build different books

There are some CLI arguments that you can use to compile books other than the default (`trpl`). This repository also includes the Rustonomicon.

You can build it like this:

```sh
$ cargo run --release -- --prefix=nomicon --source=nomicon --meta=nomicon_meta.yml
```

## License

The books are Copyright (c) 2015 The Rust Project Developers and licensed ([MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) and [Apache](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)).

The code is licensed the same as the books, based off the [original code](https://github.com/killercup/trpl-ebook) from Pascal Hertleif (killercup).
