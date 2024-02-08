use crate::std_msgs;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Accel {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AccelStamped {
    pub header: std_msgs::Header,
    pub accel: Accel,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Inertia {
    pub m: f64,
    pub com: Vector3,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct InertiaStamped {
    pub header: std_msgs::Header,
    pub inertia: Inertia,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct PointStamped {
    pub header: std_msgs::Header,
    pub point: Point,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Pose2D {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TransformStamped {
    pub header: std_msgs::Header,
    pub child_frame_id: String,
    pub transform: Transform,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TwistStamped {
    pub header: std_msgs::Header,
    pub twist: Twist,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
