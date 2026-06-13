# vcf

A small Rust tool that reorders **wordlists and unordered, text-only sets** so they
compress better. It targets collections where line order is irrelevant — for example
password wordlists like `rockyou.txt`.

`vcf` is **not** a general-purpose archiver: it only handles plain text, and only when
the order of the lines doesn't matter.

## How it works

Each line becomes a `Node`. Starting from a seed word, `vcf` greedily picks the next
word that maximizes the length of shared **contiguous substrings** with the current one
(a similarity score that rewards longer common segments). Because similar strings end
up next to each other, the reordered stream compresses much better when piped through a
general-purpose compressor (gzip / zstd / xz) than the original ordering would.

## Build

```sh
cargo build --release
```

The release profile is tuned for throughput: `lto = "fat"`, `codegen-units = 1`,
`panic = "abort"` and a stripped binary.

## Usage

The current version reads `rockyou.txt` from the working directory and writes the
reordered output to `rockyou.vcf`:

```sh
cargo run --release
```

## Wordlists

`rockyou.txt` and `rockyou_sorted.txt` exceed GitHub's 100 MB file limit, so they are
committed in ~90 MB parts. Reassemble them before running:

```sh
cat rockyou.txt.part*        > rockyou.txt
cat rockyou_sorted.txt.part* > rockyou_sorted.txt
```

## License

Released under the [MIT License](LICENSE).
