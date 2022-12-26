use std::{fmt, str::FromStr};

use cid::Cid;
use unsigned_varint::encode as varint;

use crate::{util, *};

/// Commit identifier, includes type, genesis CID, commit CID.
///
/// Encoded as `<multibase-prefix><multicodec-streamid><type><genesis-cid-bytes><commit-cid-bytes>`.
///
/// String representation is base36-encoding of the bytes above.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommitId {
    pub stream_type: StreamType,
    pub cid: Cid,
    pub commit: Option<Cid>,
}

impl CommitId {
    /// Get the commit CID.
    pub fn commit(&self) -> Cid {
        self.commit.unwrap_or(self.cid)
    }

    /// Parse from slice of bytes.
    pub fn from_slice<I: AsRef<[u8]>>(value: I) -> Result<Self> {
        util::try_from_slice::<true, false>(value.as_ref()).map(|stream_ref| {
            if let StreamRef::CommitId(commit_id) = stream_ref {
                commit_id
            } else {
                unreachable!()
            }
        })
    }
}

impl StreamRefExt for CommitId {
    fn stream_type(&self) -> StreamType {
        self.stream_type
    }

    fn cid(&self) -> &Cid {
        &self.cid
    }

    fn at_commit(&self, commit: Cid) -> Self {
        Self {
            stream_type: self.stream_type,
            cid: self.cid,
            commit: Some(commit),
        }
    }

    fn to_base_id(&self) -> StreamId {
        StreamId {
            stream_type: self.stream_type,
            cid: self.cid,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut codec_buf = varint::u8_buffer();
        let codec = varint::u8(STREAMID_CODEC, &mut codec_buf);

        let mut stream_type_buf = varint::u8_buffer();
        let stream_type = varint::u8(self.stream_type.into(), &mut stream_type_buf);

        let cid_bytes = self.cid.to_bytes();

        let commit_bytes = self
            .commit
            .as_ref()
            .map(Cid::to_bytes)
            .unwrap_or_else(|| vec![0]);

        [codec, stream_type, &cid_bytes, &commit_bytes].concat()
    }
}

impl fmt::Display for CommitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base36_string())
    }
}

impl FromStr for CommitId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        util::from_str::<true, false>(s).map(|stream_ref| {
            if let StreamRef::CommitId(commit_id) = stream_ref {
                commit_id
            } else {
                unreachable!()
            }
        })
    }
}
