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

Export `RUSTFLAGS="-C target-cpu=native"` first to increase speed at the cost of
portability.

Alternatively, a very small (< 3 MB) Docker image can be built using the
included Dockerfile.

## Run

```shell
cargo run --release
```

Or just run the `they-fight-crime` executable in `target/release` directly.

The default port is `8080`, and the default JSON file path is `./data.json`.
These can be set with CLI parameters, run with the `--help` parameter to find
out more.

## Performance

Benchmarking was performed using [vegeta](https://github.com/tsenart/vegeta) on
a headless Debian Stretch machine with an Intel Core i7-6700K and 8 GB of RAM.

At idle, CPU usage was 0% and RAM usage was 3.4 MB.

```
$ ulimit -n 40000 # An arbitrarily large number.
$ /usr/bin/time -v target/release/they-fight-crime &
$ echo "GET http://127.0.0.1:8080/" | vegeta attack -duration 10s -rate 50000/1s | tee tfc.50k.10s.bin | vegeta report
Requests      [total, rate]            499998, 50000.08
Duration      [total, attack, wait]    9.999995309s, 9.999943092s, 52.217µs
Latencies     [mean, 50, 95, 99, max]  71.236µs, 56.44µs, 112.075µs, 393.861µs, 47.324202ms
Bytes In      [total, mean]            72431277, 144.86
Bytes Out     [total, mean]            0, 0.00
Success       [ratio]                  100.00%
Status Codes  [code:count]             200:499998
Error Set:
```

Under load, max CPU usage was 90% and max RAM usage was 8.7 MB.

CPU usage was as seen in top with a 1s refresh interval, so may be inaccurate.
The load test was run several times, restarting they-fight-crime each time, and
the results were roughly the same each time (however, the 95%, 99% and max
latencies varied between 1/3 and 3x the values above).

At higher rates, vegeta started to use > 750% CPU, starving they-fight-crime of
the CPU it needed, and causing requests to fail.
