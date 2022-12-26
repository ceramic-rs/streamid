use std::str::FromStr;

use cid::{multibase::decode, Cid};
use once_cell::sync::Lazy;
use streamid::*;

const BASE_CID_STRING: &str = "bagcqcerakszw2vsovxznyp5gfnpdj4cqm2xiv76yd24wkjewhhykovorwo6a";
static BASE_CID: Lazy<Cid> = Lazy::new(|| Cid::from_str(BASE_CID_STRING).unwrap());
const COMMIT_CID_STRING: &str = "bagjqcgzaday6dzalvmy5ady2m5a5legq5zrbsnlxfc2bfxej532ds7htpova";
static COMMIT_CID: Lazy<Cid> = Lazy::new(|| Cid::from_str(COMMIT_CID_STRING).unwrap());

const STREAM_ID_STRING: &str = "kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";
static STREAM_ID_BYTES: Lazy<Vec<u8>> = Lazy::new(|| decode(STREAM_ID_STRING).unwrap().1);
const STREAM_ID_URL: &str =
    "ceramic://kjzl6cwe1jw147dvq16zluojmraqvwdmbh61dx9e0c59i344lcrsgqfohexp60s";
const STREAM_ID_URL_LEGACY: &str =
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
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *BASE_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_0_COMMIT);

    let base_id = commit_id.to_base_id();

    assert_eq!(base_id.stream_type(), StreamType::Tile);
    assert_eq!(base_id.cid(), &*BASE_CID);
}

#[test]
fn new_at_commit() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: Some(*COMMIT_CID),
    };

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *COMMIT_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_COMMIT);

    let base_id = commit_id.to_base_id();

    assert_eq!(base_id.stream_type(), StreamType::Tile);
    assert_eq!(base_id.cid(), &*BASE_CID);
}

#[test]
fn from_bytes_err() {
    assert!(CommitId::from_slice(STREAM_ID_BYTES.as_slice()).is_err());
    assert!(CommitId::from_slice(BASE_CID.to_bytes()).is_err());
}

#[test]
fn from_bytes() {
    let commit_id = CommitId::from_slice(STREAM_ID_WITH_0_COMMIT_BYTES.as_slice()).unwrap();

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *BASE_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_0_COMMIT);
    assert_eq!(
        commit_id.to_base_id(),
        StreamId {
            stream_type: StreamType::Tile,
            cid: *BASE_CID
        }
    );
}

#[test]
fn from_bytes_with_commit() {
    let commit_id = CommitId::from_slice(STREAM_ID_WITH_COMMIT_BYTES.as_slice()).unwrap();

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *COMMIT_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_COMMIT);
    assert_eq!(
        commit_id.to_base_id(),
        StreamId {
            stream_type: StreamType::Tile,
            cid: *BASE_CID
        }
    );
}

#[test]
fn from_bytes_roundtrip() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    let commit_id2 = CommitId::from_slice(commit_id.to_bytes()).unwrap();

    assert_eq!(commit_id.to_string(), commit_id2.to_string());
    assert_eq!(commit_id.to_bytes(), commit_id2.to_bytes());
}

#[test]
fn from_bytes_roundtrip_with_commit() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: Some(*COMMIT_CID),
    };

    let commit_id2 = CommitId::from_slice(commit_id.to_bytes()).unwrap();

    assert_eq!(commit_id.to_string(), commit_id2.to_string());
    assert_eq!(commit_id.to_bytes(), commit_id2.to_bytes());
}

#[test]
fn from_string_err() {
    assert!(CommitId::from_str(STREAM_ID_STRING).is_err());
    assert!(CommitId::from_str(STREAM_ID_URL).is_err());
    assert!(CommitId::from_str(STREAM_ID_URL_LEGACY).is_err());
}

#[test]
fn from_string() {
    let commit_id = CommitId::from_str(STREAM_ID_WITH_0_COMMIT).unwrap();

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *BASE_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_0_COMMIT);
    assert_eq!(
        commit_id.to_base_id(),
        StreamId {
            stream_type: StreamType::Tile,
            cid: *BASE_CID
        }
    );
}

#[test]
fn from_string_with_commit() {
    let commit_id = CommitId::from_str(STREAM_ID_WITH_COMMIT).unwrap();

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *COMMIT_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_COMMIT);
    assert_eq!(
        commit_id.to_base_id(),
        StreamId {
            stream_type: StreamType::Tile,
            cid: *BASE_CID
        }
    );
}

#[test]
fn from_string_legacy() {
    let commit_id = CommitId::from_str(STREAM_ID_WITH_0_COMMIT_LEGACY).unwrap();

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *BASE_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_0_COMMIT);
    assert_eq!(
        commit_id.to_base_id(),
        StreamId {
            stream_type: StreamType::Tile,
            cid: *BASE_CID
        }
    );
}

#[test]
fn from_string_with_commit_legacy() {
    let commit_id = CommitId::from_str(STREAM_ID_WITH_COMMIT_LEGACY).unwrap();

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *COMMIT_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_COMMIT);
    assert_eq!(
        commit_id.to_base_id(),
        StreamId {
            stream_type: StreamType::Tile,
            cid: *BASE_CID
        }
    );
}

#[test]
fn from_string_roundtrip() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };
    let commit_id2 = CommitId::from_str(&commit_id.to_string()).unwrap();
    assert_eq!(commit_id.to_string(), commit_id2.to_string());
}

#[test]
fn from_string_roundtrip_with_commit() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: Some(*COMMIT_CID),
    };
    let commit_id2 = CommitId::from_str(&commit_id.to_string()).unwrap();
    assert_eq!(commit_id.to_string(), commit_id2.to_string());
}

#[test]
fn at_commit() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    let commit_id = commit_id.at_commit(*COMMIT_CID);

    assert_eq!(commit_id.stream_type(), StreamType::Tile);
    assert_eq!(commit_id.cid(), &*BASE_CID);
    assert_eq!(commit_id.commit(), *COMMIT_CID);
    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_COMMIT);

    let base_id = commit_id.to_base_id();

    assert_eq!(base_id.stream_type(), StreamType::Tile);
    assert_eq!(base_id.cid(), &*BASE_CID);
}

#[test]
fn to_bytes() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    assert_eq!(commit_id.to_bytes(), *STREAM_ID_WITH_0_COMMIT_BYTES);
}

#[test]
fn to_string() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    assert_eq!(commit_id.to_string(), STREAM_ID_WITH_0_COMMIT);
}

#[test]
fn to_url() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    assert_eq!(
        commit_id.to_url(),
        format!("ceramic://{STREAM_ID_WITH_0_COMMIT}")
    );
}

#[test]
fn eq() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    let commit_id2 = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: None,
    };

    let commit_id3 = CommitId {
        stream_type: StreamType::Caip10Link,
        cid: *BASE_CID,
        commit: None,
    };

    assert_eq!(commit_id, commit_id2);
    assert_ne!(commit_id, commit_id3);
}

#[test]
fn to_base_id() {
    let commit_id = CommitId {
        stream_type: StreamType::Tile,
        cid: *BASE_CID,
        commit: Some(*COMMIT_CID),
    };
    let stream_id = commit_id.to_base_id();

    assert_eq!(stream_id.stream_type(), commit_id.stream_type());
    assert_eq!(stream_id.cid(), commit_id.cid());
}
