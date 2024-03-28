use crate::std_msgs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DmaBuf {
    pub header: std_msgs::Header,
    pub pid: u32,
    pub fd: i32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub fourcc: u32,
    pub length: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RadarCube {
    pub header: std_msgs::Header,
    pub timestamp: u64,
    pub frame_id: u32,
    pub layout: Vec<u8>,
    pub shape: Vec<u16>,
    pub scales: Vec<f32>,
    pub cube: Vec<i16>,
    pub is_complex: bool,
}

pub mod radar_cube_dimension {
    pub const UNDEFINED: u8 = 0;
    pub const RANGE: u8 = 1;
    pub const DOPPLER: u8 = 2;
    pub const AZIMUTH: u8 = 3;
    pub const ELEVATION: u8 = 4;
    pub const RXCHANNEL: u8 = 5;
    pub const SEQUENCE: u8 = 6;
}
