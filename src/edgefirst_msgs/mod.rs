use crate::{builtin_interfaces::Time, std_msgs};
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

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DeepviewDetectBoxes2D {
    pub header: std_msgs::Header,
    pub input_timestamp: Time,
    pub model_tile: Time,
    pub output_time: Time,
    pub boxes: Vec<DeepviewDetectBox2D>,
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DeepviewDetectBox2D {
    pub center_x: f32,
    pub center_y: f32,
    pub width: f32,
    pub height: f32,
    pub label: String,
    pub score: f32,
    pub distance: f32,
    pub speed: f32,
    pub is_tracked: bool,
    pub track: DeepviewDetectTrack,
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DeepviewDetectTrack {
    pub id: String,
    pub lifetime: i32,
    pub created: Time,
}
