use crate::service::ServiceHeader;
use serde_derive::{Deserialize, Serialize};

pub mod response_status {
    pub const SUCCESS: u32 = 1;
    pub const IGNORED: u32 = 2;
    pub const WARN: u32 = 3;
    pub const ERROR: u32 = 4;
}
#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ResponseStatus {
    pub code: u32,
    pub message: String,
}

// -----service-----

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EngageRequest {
    pub mode: bool,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EngageResponse {
    pub status: ResponseStatus,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RawEngageRequest {
    pub header: ServiceHeader,
    pub mode: bool,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RawEngageResponse {
    pub header: ServiceHeader,
    pub status: ResponseStatus,
}
