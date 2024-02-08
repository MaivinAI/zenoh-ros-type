use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Time {
    pub sec: i32,
    pub nanosec: u32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Duration {
    pub sec: i32,
    pub nanosec: u32,
}
