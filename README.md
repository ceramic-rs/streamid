# Ceramic StreamID

![License](https://img.shields.io/crates/l/streamid)
[![Crates.io](https://img.shields.io/crates/v/streamid)][crates-io]

[crates-io]: https://crates.io/crates/streamid

> This package contains Ceramic StreamID and CommitID implementation.

Implements Ceramic streamIDs as defined in ceramic spec and
[CIP](https://github.com/ceramicnetwork/CIP/blob/master/CIPs/CIP-59/CIP-59.md),
represented as [`StreamId`] and [`StreamId`] for API clarity.

[`StreamId`] represents a reference to a stream as a whole, thus does not
contain commit information.

[`CommitId`] represents a reference to a particular commit in the stream
evolution.

```text
<streamid> ::= <multibase-prefix><multicodec-streamid><type><genesis-cid-bytes>
```

or including [`StreamId`] commit

```text
<streamid> ::= <multibase-prefix><multicodec-streamid><type><genesis-cid-bytes><commit-cid-bytes>
```

## Getting started

### Installation

```shell
$ cargo add streamid
```

### Usage

See the [ceramic developer site](https://developers.ceramic.network/) for more
details about how to use this package.

To reference a stream as a whole, use [`StreamId`]. You can create an instance
from the parts. stream type string or integer and CID instance or string are
required.

```rust
use std::str::FromStr;

use cid::Cid;
use streamid::*;

const CID_STRING: &str = "bagcqcerakszw2vsovxznyp5gfnpdj4cqm2xiv76yd24wkjewhhykovorwo6a";
const STREAM_ID_STRING: &str = "kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";

let stream_id = StreamId::from_str(STREAM_ID_STRING).unwrap();

assert_eq!(stream_id.stream_type(), StreamType::Tile);
assert_eq!(stream_id.stream_type().to_string(), "tile");
assert_eq!(stream_id.cid().to_string(), CID_STRING);
assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
assert_eq!(stream_id.to_url(), format!("ceramic://{STREAM_ID_STRING}"));
```

You can also create `StreamId` instance from `StreamId` string or bytes.

```rust
use std::str::FromStr;

use streamid::*;

const STREAM_ID_STRING: &str = "kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";

let stream_id = StreamId::from_str(STREAM_ID_STRING).unwrap();
```

```rust,no_run
use streamid::*;

let stream_id = StreamId::from_slice(vec![]).unwrap();
```

To reference particular point in a stream evolution, use [`CommitId`]. In
addition to stream type ([`StreamType`]) and genesis reference ([`Cid`]), one is
expected to provide a reference to commit ([`Cid`]). If you pass [`None`], this
would reference a genesis commit.

```rust
use std::str::FromStr;

use streamid::*;

const BASE_CID_STRING: &str = "bagcqcerakszw2vsovxznyp5gfnpdj4cqm2xiv76yd24wkjewhhykovorwo6a";
const COMMIT_CID_STRING: &str = "bagjqcgzaday6dzalvmy5ady2m5a5legq5zrbsnlxfc2bfxej532ds7htpova";
const COMMIT_ID_STRING: &str =
  "k1dpgaqe3i64kjqcp801r3sn7ysi5i0k7nxvs7j351s7kewfzr3l7mdxnj7szwo4kr9mn2qki5nnj0cv836ythy1t1gya9s25cn1nexst3jxi5o3h6qprfyju";

let commit_id = CommitId::from_str(COMMIT_ID_STRING).unwrap();

assert_eq!(commit_id.stream_type(), StreamType::Tile);
assert_eq!(commit_id.stream_type().to_string(), "tile");
assert_eq!(commit_id.cid().to_string(), BASE_CID_STRING);
assert_eq!(commit_id.commit().to_string(), COMMIT_CID_STRING);
assert_eq!(commit_id.to_string(), COMMIT_ID_STRING);
assert_eq!(commit_id.to_url(), format!("ceramic://{COMMIT_ID_STRING}"));
```

To reference specific CID from [`StreamId`] or to change commit reference in
[`CommitId`], use [`StreamRefExt::at_commit`] method:

```rust,ignore
use streamid::StreamRefExt;

commit_id.at_commit("bagcqcerakszw2vsov..."); // #=> new CommitId for the same stream
stream_id.at_commit("bagcqcerakszw2vsov..."); // #=> new CommitId for the same stream
```

[`CommitId`] ([`StreamId`] for compatibility also) can get you base [`StreamId`]
via [`StreamRefExt::to_base_id`]:

```rust,ignore
use streamid::StreamRefExt;

commit_id.to_base_id(); // #=> StreamID reference to the stream
stream_id.to_base_id(); // #=> new StreamID reference to the same stream, effectively a shallow clone.
```

To parse an unknown input into proper [`CommitId`] or [`StreamId`], you could
use `StreamRef::from_str`:

```rust,ignore
use std::str::FromStr;

use streamid::StreamRef;

let input = "bagcqcerakszw2vsov...";
let stream_id_or_commit_id = StreamRef::from_str(input).unwrap();
```

## Development

Run tests:

```shell
cargo test
```

Run linter:

```shell
cargo clippy
cargo fmt
```

## Contributing

We are happy to accept small and large contributions. Make sure to check out the
[Ceramic
specifications](https://github.com/ceramicnetwork/ceramic/blob/main/SPECIFICATION.md)
for details of how the protocol works.

## License

MIT or Apache-2.0
