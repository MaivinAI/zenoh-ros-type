use crate::std_msgs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DeepviewDMABuf {
    pub header: std_msgs::Header,
    pub src_pid: u32,
    pub dma_fd: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub fourcc: u32,
}
