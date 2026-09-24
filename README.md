# selector-db

A curated 4-byte function selector registry, searched in both directions.

Four bytes is not much entropy. Different signatures really do collide, which is
why services like 4byte.directory exist and why any honest lookup returns a
*candidate list* rather than a single answer. This crate does the same thing for
the selectors that show up in almost every transaction you will read.

## Usage

```rust
use selector_db::{signatures_for, selector_for, selector_of_calldata};

assert_eq!(signatures_for("0xa9059cbb"), vec!["transfer(address,uint256)"]);
assert_eq!(selector_for("balanceOf(address)"), Some("0x70a08231"));
assert_eq!(selector_of_calldata("0xa9059cbb00...").as_deref(), Some("0xa9059cbb"));
```

## Why a table and not a hash function

Computing a selector means Keccak-256, which is its own dependency. Looking a
selector *up* cannot work by hashing at all — you need a table of known
signatures. Since the lookup direction is the one used when reading a transaction
from an explorer, the table is the half that matters, and it is small enough to
read in one screen.

Use the `keccak256` crate in this collection when you need to compute a selector
from a new signature.

## What it does not do

- **No exhaustive database.** Twenty common signatures, not the millions in
  4byte.directory. Unknown selectors return an empty list, never a guess.
- **No collision resolution.** When two signatures share a selector, both are
  returned and the caller decides from the argument shape.
- **No parameter decoding.** `evm-calldata` handles that.

## Development

```bash
cargo test
```

## License

MIT
