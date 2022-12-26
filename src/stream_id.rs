use std::{fmt, str::FromStr};

use cid::{
    multihash::{Code, MultihashDigest},
    Cid,
};
use libipld::{cbor::DagCborCodec, prelude::*};
use unsigned_varint::encode as varint;

use crate::{util, *};

/// Stream identifier, no commit information included.
///
/// Contains stream type and CID of genesis commit.
///
/// Encoded as `<multibase-prefix><multicodec-streamid><type><genesis-cid-bytes>`.
///
/// String representation is base36-encoding of the bytes above.
///
/// ```rust
/// # use std::str::FromStr;
/// #
/// # use streamid::*;
/// let cid = cid::Cid::from_str("bagcqcerakszw2vsovxznyp5gfnpdj4cqm2xiv76yd24wkjewhhykovorwo6a").unwrap();
/// let _stream_id = StreamId { stream_type: StreamType::Tile, cid };
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StreamId {
    pub stream_type: StreamType,
    pub cid: Cid,
}

impl StreamId {
    /// Create a [`StreamId`] from a genesis commit.
    ///
    /// ```rust
    /// # use streamid::*;
    /// let genesis = Ipld::Map(
    ///     [(
    ///         "header".into(),
    ///         Ipld::Map(
    ///             [
    ///                 (
    ///                     "controllers".into(),
    ///                     Ipld::List(vec!["did:3:kjz...".into()]),
    ///                 ),
    ///                 ("family".into(), "IDX".into()),
    ///             ]
    ///             .into_iter()
    ///             .collect(),
    ///         ),
    ///     )]
    ///     .into_iter()
    ///     .collect(),
    /// );
    ///
    /// let _stream_id = StreamId::from_genesis(StreamType::Tile, &genesis);
    /// ```
    pub fn from_genesis(stream_type: StreamType, genesis: &Ipld) -> Result<Self> {
        let bytes: Vec<u8> = DagCborCodec
            .encode(genesis)
            .map_err(|err| Error::CborEncoding(err.to_string()))?;
        let hash = Code::Sha2_256.digest(&bytes);
        let cid = Cid::new_v1(DagCborCodec.into(), hash);
        Ok(StreamId { stream_type, cid })
    }

    pub fn from_slice<I: AsRef<[u8]>>(value: I) -> Result<Self> {
        util::try_from_slice::<false, true>(value.as_ref()).map(|stream_ref| {
            if let StreamRef::StreamId(stream_id) = stream_ref {
                stream_id
            } else {
                unreachable!()
            }
        })
    }
}

impl StreamRefExt for StreamId {
    fn stream_type(&self) -> StreamType {
        self.stream_type
    }

    fn cid(&self) -> &Cid {
        &self.cid
    }

    fn at_commit(&self, commit: Cid) -> CommitId {
        CommitId {
            stream_type: self.stream_type,
            cid: self.cid,
            commit: Some(commit),
        }
    }

    fn to_base_id(&self) -> StreamId {
        self.clone()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut codec_buf = varint::u8_buffer();
        let codec = varint::u8(STREAMID_CODEC, &mut codec_buf);

        let mut stream_type_buf = varint::u64_buffer();
        let stream_type = varint::u64(self.stream_type as u64, &mut stream_type_buf);

        let cid_bytes = self.cid.to_bytes();

        [codec, stream_type, &cid_bytes].concat()
    }
}

impl fmt::Display for StreamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base36_string())
    }
}

impl FromStr for StreamId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        util::from_str::<false, true>(s).map(|stream_ref| {
            if let StreamRef::StreamId(stream_id) = stream_ref {
                stream_id
            } else {
                unreachable!()
            }
        })
    }
}
