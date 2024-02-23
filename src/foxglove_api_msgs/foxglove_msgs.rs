use crate::{
    builtin_interfaces,
    std_msgs::{self},
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxgloveCompressedVideo {
    pub header: std_msgs::Header,
    pub data: Vec<u8>,
    pub format: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxgloveImageAnnotations {
    pub circles: Vec<FoxgloveCircleAnnotations>,
    pub points: Vec<FoxglovePointAnnotations>,
    pub texts: Vec<FoxgloveTextAnnotations>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxgloveCircleAnnotations {
    pub timestamp: builtin_interfaces::Time,
    pub position: FoxglovePoint2,
    pub diameter: f64,
    pub thickness: f64,
    pub fill_color: std_msgs::ColorRGBA,
    pub outline_color: std_msgs::ColorRGBA,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxglovePointAnnotations {
    pub timestamp: builtin_interfaces::Time,
    pub type_: u8,
    pub points: Vec<FoxglovePoint2>,
    pub outline_color: std_msgs::ColorRGBA,
    pub outline_colors: Vec<std_msgs::ColorRGBA>,
    pub fill_color: std_msgs::ColorRGBA,
    pub thickness: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxgloveTextAnnotations {
    pub timestamp: builtin_interfaces::Time,
    pub position: FoxglovePoint2,
    pub text: String,
    pub font_size: f64,
    pub text_color: std_msgs::ColorRGBA,
    pub background_color: std_msgs::ColorRGBA,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct FoxglovePoint2 {
    pub x: f64,
    pub y: f64,
}
