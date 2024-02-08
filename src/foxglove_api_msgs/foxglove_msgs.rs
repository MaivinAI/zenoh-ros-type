use crate::std_msgs;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxgloveCompressedVideo {
    pub header: std_msgs::Header,
    pub data: Vec<u8>,
    pub format: String,
}
