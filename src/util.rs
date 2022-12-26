use std::{io, str::FromStr};

use cid::{
    multibase::{decode, encode, Base},
    Cid,
};
use once_cell::sync::Lazy;
use regex::Regex;
use unsigned_varint::decode::u8 as decode_u8;

use crate::*;

// RegExp to match against URL representation of StreamID.
static URL_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?:ceramic://|/ceramic/)?([a-zA-Z0-9]+)$").unwrap());

// RegExp to match against URL representation of StreamID CommitID.
static URL_PATTERN_COMMIT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:ceramic://|/ceramic/)?([a-zA-Z0-9]+)(?:\?commit=([a-zA-Z0-9]+))?$").unwrap()
});

pub fn from_str<const COMMIT_ID: bool, const STREAM_ID: bool>(s: &str) -> Result<StreamRef> {
    let err = || {
        if COMMIT_ID && STREAM_ID {
            Error::InvalidStreamRefString(s.into())
        } else if COMMIT_ID {
            Error::InvalidCommitIdString(s.into())
        } else {
            Error::InvalidStreamIdString(s.into())
        }
    };

    let protocol_match = if COMMIT_ID {
        &*URL_PATTERN_COMMIT
    } else {
        &*URL_PATTERN
    }
    .captures(s)
    .ok_or_else(err)?;

    let base = protocol_match.get(1).map(|m| m.as_str()).ok_or_else(err)?;
    let (_, bytes) = decode(base).map_err(|_| err())?;

    let mut stream_ref = StreamRef::from_slice(bytes)?;
    let commit = protocol_match.get(2);

    if COMMIT_ID && commit.is_some() {
        let commit = commit
            .map(|m| m.as_str())
            .and_then(|commit| Cid::from_str(commit).ok());

        stream_ref = match stream_ref {
            StreamRef::CommitId(CommitId {
                stream_type, cid, ..
            }) => StreamRef::CommitId(CommitId {
                stream_type,
                cid,
                commit: commit.filter(|commit| commit != &cid),
            }),
            StreamRef::StreamId(StreamId { stream_type, cid }) => StreamRef::CommitId(CommitId {
                stream_type,
                cid,
                commit,
            }),
        };
    }

    match stream_ref {
        StreamRef::CommitId(_) if COMMIT_ID => {}
        StreamRef::StreamId(_) if STREAM_ID => {}
        _ => return Err(err()),
    }

    Ok(stream_ref)
}

pub fn try_from_slice<const COMMIT_ID: bool, const STREAM_ID: bool>(
    buf: &[u8],
) -> Result<StreamRef> {
    let (stream_codec, buf) = decode_u8(buf)?;
    if stream_codec != STREAMID_CODEC {
        return Err(Error::InvalidStreamRefCodec);
    }

    let (stream_type, buf) = decode_u8(buf)?;
    let stream_type = StreamType::try_from(stream_type)?;

    let (cid, buf) = read_cid(buf)?;

    if STREAM_ID && buf.is_empty() {
        Ok(StreamRef::StreamId(StreamId { stream_type, cid }))
    } else if !COMMIT_ID {
        Err(Error::InvalidStreamIdBytes(encode(Base::Base36Lower, buf)))
    } else if buf.len() == 1 && buf[0] == 0 {
        // Zero commit
        Ok(StreamRef::CommitId(CommitId {
            stream_type,
            cid,
            commit: None,
        }))
    } else {
        // Commit
        let commit = Cid::read_bytes(buf)?;
        Ok(StreamRef::CommitId(CommitId {
            stream_type,
            cid,
            commit: Some(commit),
        }))
    }
}

pub fn read_cid(bytes: &[u8]) -> Result<(Cid, &[u8])> {
    let mut reader = io::Cursor::new(bytes);
    let cid = Cid::read_bytes(&mut reader)?;
    let bytes = &bytes[reader.position() as usize..];
    Ok((cid, bytes))
}
