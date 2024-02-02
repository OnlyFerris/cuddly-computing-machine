# cuddly-computing-machine

The cuddliest computing machine in the world, ever!

## Run it

```zsh
cargo run
```

## Resources

1. https://docs.rust-embedded.org/book/intro/no-std.html
1. https://doc.rust-lang.org/nomicon/beneath-std.html 

## MSRV – Minimum-Supported Rust Version

Probably `nightly`.

```zsh
rustup install nightly
rustup default nightly
```

But you can double check that using [cargo-msrv](https://crates.io/crates/cargo-msrv).
https://stackoverflow.com/questions/74133413/list-minimum-rust-version-required-for-rust-project/74133640#74133640

```zsh
cargo msrv # assuming you've already installed cargo-msrv
```

It'll try to use different versions of the Rust toolchain.

Most likely it'll end with:

```text
Unable to find a Minimum Supported Rust Version (MSRV).

If you think this result is erroneous, please run: `cargo check` manually.

If the above does succeed, or you think cargo-msrv errored in another way, please feel free to
report the issue at: https://github.com/foresterre/cargo-msrv/issues

Thank you in advance!
```

In which case, `nightly` is still the MSRV.

But maybe you are from the far future, and `nightly` is no longer needed then.
Today is Fri 2 Feb 2024. It's 1:30 AM.
