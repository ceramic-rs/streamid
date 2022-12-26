use thiserror::Error;

use crate::StreamType;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing CommitID from bytes {0}: no commit information provided")]
    InvalidCommitIdBytes(String),

    #[error("Error while parsing CommitID from string {0}: no commit information provided")]
    InvalidCommitIdString(String),

    #[error("Invalid StreamID bytes {0}: contains commit")]
    InvalidStreamIdBytes(String),

    #[error("Invalid StreamID string {0}: contains commit")]
    InvalidStreamIdString(String),

    #[error("Invalid StreamRef bytes: {0}")]
    InvalidStreamRefBytes(String),

    #[error("Invalid StreamRef, does not include streamid codec")]
    InvalidStreamRefCodec,

    #[error("Invalid StreamRef string: {0}")]
    InvalidStreamRefString(String),

    #[error("Invalid StreamType index: {0}")]
    InvalidStreamTypeIndex(#[from] num_enum::TryFromPrimitiveError<StreamType>),

    #[error("Invalid StreamType name: {0}")]
    InvalidStreamTypeName(#[from] serde_plain::Error),

    #[error("Unknown CID version {0}")]
    UnknownCidVersion(u64),

    #[error(transparent)]
    Cbor(#[from] dag_cbor::CborError),

    #[error(transparent)]
    Cid(#[from] cid::Error),

    #[error(transparent)]
    Multihash(#[from] cid::multihash::Error),

    #[error(transparent)]
    VarintDecode(#[from] unsigned_varint::decode::Error),
}
