use crate::std_msgs::Header;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HADMapBin {
    pub header: Header,
    pub map_format: u8,
    pub format_version: String,
    pub map_version: String,
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MapPrimitive {
    pub id: i64,
    pub primitive_type: String,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HADMapSegment {
    pub primitives: Vec<MapPrimitive>,
    pub preferred_primitive_id: i64,
}
