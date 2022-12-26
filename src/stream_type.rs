use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum StreamType {
    #[serde(rename = "tile")]
    Tile = 0,

    #[serde(rename = "caip10-link")]
    Caip10Link = 1,

    #[serde(rename = "model")]
    Model = 2,

    #[serde(rename = "MID")]
    Mid = 3,

    #[serde(rename = "UNLOADABLE")]
    Unloadable = 4,
}

derive_display_from_serialize!(StreamType);
derive_fromstr_from_deserialize!(StreamType);
