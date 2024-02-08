use crate::builtin_interfaces::Time;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AckermannControlCommand {
    pub stamp: Time,
    pub lateral: AckermannLateralCommand,
    pub longitudinal: LongitudinalCommand,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AckermannLateralCommand {
    pub stamp: Time,
    pub steering_tire_angle: f32,
    pub steering_tire_rotation_rate: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HighLevelControlCommand {
    pub stamp: Time,
    pub velocity_mps: f32,
    pub curvature: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct LongitudinalCommand {
    pub stamp: Time,
    pub speed: f32,
    pub acceleration: f32,
    pub jerk: f32,
}
