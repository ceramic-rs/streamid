use std::{fmt, str::FromStr};

use cid::{
    multibase::{encode, Base},
    Cid,
};

use crate::{util, *};

/// A [`CommitId`] or [`StreamId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StreamRef {
    CommitId(CommitId),
    StreamId(StreamId),
}

impl StreamRef {
    pub fn from_slice<I: AsRef<[u8]>>(value: I) -> Result<Self> {
        util::try_from_slice::<true, true>(value.as_ref())
    }
}

pub trait StreamRefExt {
    /// Get the [`StreamType`].
    fn stream_type(&self) -> StreamType;

    /// Get the genesis [`Cid`].
    fn cid(&self) -> &Cid;

    /// Create a new [`CommitId`] at the given commit.
    fn at_commit(&self, commit: Cid) -> CommitId;

    /// Get the [`StreamId`] without the commit.
    fn to_base_id(&self) -> StreamId;

    /// Encode the [`StreamRef`] into bytes.
    fn to_bytes(&self) -> Vec<u8>;

    /// Encode the [`StreamRef`] into a string.
    fn to_base36_string(&self) -> String {
        encode(Base::Base36Lower, self.to_bytes())
    }

    /// Encode the [`StreamRef`] into a base36 URL.
    fn to_url(&self) -> String {
        format!("ceramic://{}", self.to_base36_string())
    }
}

impl StreamRefExt for StreamRef {
    fn stream_type(&self) -> StreamType {
        match self {
            StreamRef::StreamId(stream_id) => stream_id.stream_type(),
            StreamRef::CommitId(commit_id) => commit_id.stream_type(),
        }
    }

    fn cid(&self) -> &Cid {
        match self {
            StreamRef::StreamId(stream_id) => stream_id.cid(),
            StreamRef::CommitId(commit_id) => commit_id.cid(),
        }
    }

    fn at_commit(&self, commit: Cid) -> CommitId {
        match self {
            StreamRef::StreamId(stream_id) => stream_id.at_commit(commit),
            StreamRef::CommitId(commit_id) => commit_id.at_commit(commit),
        }
    }

    fn to_base_id(&self) -> StreamId {
        match self {
            StreamRef::StreamId(stream_id) => stream_id.to_base_id(),
            StreamRef::CommitId(commit_id) => commit_id.to_base_id(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            StreamRef::StreamId(stream_id) => stream_id.to_bytes(),
            StreamRef::CommitId(commit_id) => commit_id.to_bytes(),
        }
    }
}

impl fmt::Display for StreamRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base36_string())
    }
}

impl FromStr for StreamRef {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        util::from_str::<true, true>(s)
    }
}
