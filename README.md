They Fight Crime!
=================

Quite possibly the fastest buddy cop show tagline generation server in the
world! (How much competition can there be?) Written in
[Rust](https://www.rust-lang.org/) and built on top of
[actix-web](https://actix.rs/), it responds to `GET /` requests with text of the
form

> One's a(n) A B with C, the other's a(n) X Y with Z. Together, they fight crime!

Some examples:

> One's a disgraced aristocrat with a penchant for trickery, the other's a skydiving farrier with no regard for health and safety. Together, they fight crime!
>
> One's a hedonistic cat burglar with uncanny comic timing, the other's a worldly substitute teacher with a mean right hook. Together, they fight crime!
>
> One's a psychic dentist with an unfortunate stutter, the other's a killer jester with a household name. Together, they fight crime!

On startup, the server loads `data.json` from its current working directory.
When it receives a request, it picks content at random out of the nouns,
adjectives and descriptive phrases loaded to create a tagline.

**Pull requests to add more nonsense to `data.json` are welcome!**

Inspired by <https://theyfightcrime.org>.

## Build

[Rust](https://www.rust-lang.org/) must be installed to build the server.

```shell
cargo build --release
```

## Run

```shell
cargo run --release
```

Or just run the `they-fight-crime` executable in `target/release` directly (make
sure `data.json` is in the current working directory).
