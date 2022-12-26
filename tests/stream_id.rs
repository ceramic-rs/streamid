use std::str::FromStr;

use cid::{multibase::decode, Cid};
use once_cell::sync::Lazy;
use streamid::*;

const BASE_CID_STRING: &str = "bagcqcerakszw2vsovxznyp5gfnpdj4cqm2xiv76yd24wkjewhhykovorwo6a";
static BASE_CID: Lazy<Cid> = Lazy::new(|| Cid::from_str(BASE_CID_STRING).unwrap());
const COMMIT_CID_STRING: &str = "bagjqcgzaday6dzalvmy5ady2m5a5legq5zrbsnlxfc2bfxej532ds7htpova";
static COMMIT_CID: Lazy<Cid> = Lazy::new(|| Cid::from_str(COMMIT_CID_STRING).unwrap());
const STREAM_ID_STRING: &str = "kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";
const STREAM_ID_URL: &str =
    "ceramic://kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";
static STREAM_ID_BYTES: Lazy<Vec<u8>> = Lazy::new(|| decode(STREAM_ID_STRING).unwrap().1);
const STREAM_ID_LEGACY: &str =
    "/ceramic/kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";

const STREAM_ID_WITH_COMMIT: &str =
  "k1dpgaqe3i64kjqcp801r3sn7ysi5i0k7nxvs7j351s7kewfzr3l7mdxnj7szwo4kr9mn2qki5nnj0cv836ythy1t1gya9s25cn1nexst3jxi5o3h6qprfyju";
static STREAM_ID_WITH_COMMIT_BYTES: Lazy<Vec<u8>> =
    Lazy::new(|| decode(STREAM_ID_WITH_COMMIT).unwrap().1);
const STREAM_ID_WITH_0_COMMIT: &str =
    "k3y52l7qbv1frxwipl4hp7e6jlu4f6u8upm2xv0irmedfkm5cnutmezzi3u7mytj4";
static STREAM_ID_WITH_0_COMMIT_BYTES: Lazy<Vec<u8>> =
    Lazy::new(|| decode(STREAM_ID_WITH_0_COMMIT).unwrap().1);
const STREAM_ID_WITH_COMMIT_LEGACY: &str =
  "/ceramic/kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s?commit=bagjqcgzaday6dzalvmy5ady2m5a5legq5zrbsnlxfc2bfxej532ds7htpova";
const STREAM_ID_WITH_0_COMMIT_LEGACY: &str =
    "/ceramic/kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s?commit=0";

#[test]
fn new() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    assert_eq!(stream_id.stream_type(), StreamType::Tile);
    assert_eq!(stream_id.cid().to_string(), BASE_CID_STRING);
    assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
    assert_eq!(stream_id.to_base_id(), stream_id);
}

#[test]
fn from_bytes_err() {
    assert!(StreamId::from_slice(BASE_CID.to_bytes()).is_err());
    assert!(StreamId::from_slice(&*STREAM_ID_WITH_COMMIT_BYTES).is_err());
    assert!(StreamId::from_slice(&*STREAM_ID_WITH_0_COMMIT_BYTES).is_err());
}

#[test]
fn from_bytes() {
    let stream_id = StreamId::from_slice(&*STREAM_ID_BYTES).unwrap();

    assert_eq!(stream_id.stream_type(), StreamType::Tile);
    assert_eq!(stream_id.cid().to_string(), BASE_CID_STRING);
    assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
    assert_eq!(stream_id.to_base_id(), stream_id);
}

#[test]
fn from_bytes_roundtrip() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };
    let stream_id2 = StreamId::from_slice(stream_id.to_bytes()).unwrap();

    assert_eq!(stream_id.to_string(), stream_id2.to_string());
}

#[test]
fn from_str_err() {
    assert!(StreamId::from_str(STREAM_ID_WITH_COMMIT).is_err());
    assert!(StreamId::from_str(STREAM_ID_WITH_0_COMMIT).is_err());
    assert!(StreamId::from_str(STREAM_ID_WITH_COMMIT_LEGACY).is_err());
    assert!(StreamId::from_str(STREAM_ID_WITH_0_COMMIT_LEGACY).is_err());
}

#[test]
fn from_str() {
    let stream_id = StreamId::from_str(STREAM_ID_STRING).unwrap();

    assert_eq!(stream_id.stream_type(), StreamType::Tile);
    assert_eq!(stream_id.cid().to_string(), BASE_CID_STRING);
    assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
}

#[test]
fn from_str_url() {
    let stream_id = StreamId::from_str(STREAM_ID_URL).unwrap();

    assert_eq!(stream_id.stream_type(), StreamType::Tile);
    assert_eq!(stream_id.cid().to_string(), BASE_CID_STRING);
    assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
}

#[test]
fn from_str_legacy_url() {
    let stream_id = StreamId::from_str(STREAM_ID_LEGACY).unwrap();

    assert_eq!(stream_id.stream_type(), StreamType::Tile);
    assert_eq!(stream_id.cid().to_string(), BASE_CID_STRING);
    assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
}

#[test]
fn from_str_roundtrip() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };
    let stream_id2 = StreamId::from_str(&stream_id.to_string()).unwrap();

    assert_eq!(stream_id.to_string(), stream_id2.to_string());
}

#[test]
fn to_bytes() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    assert_eq!(stream_id.to_bytes(), *STREAM_ID_BYTES);
}

#[test]
fn at_commit() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    let commit_id = stream_id.at_commit(*COMMIT_CID);

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid().to_string(), BASE_CID_STRING);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_COMMIT);
    assert_eq!(commit_id.commit().to_string(), COMMIT_CID_STRING);
}

#[test]
fn to_string() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    assert_eq!(stream_id.to_string(), STREAM_ID_STRING);
}

#[test]
fn to_url() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    assert_eq!(stream_id.to_url(), format!("ceramic://{STREAM_ID_STRING}"));
}

#[test]
fn eq() {
    let stream_id = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    let stream_id2 = StreamId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
    };

    let stream_id3 = StreamId {
        stream_type: StreamType::Caip10Link,
        cid: *BASE_CID,
    };

    assert_eq!(stream_id, stream_id2);
    assert_ne!(stream_id, stream_id3);
}
