use crate::builtin_interfaces::Time;
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Clock {
    pub clock: Time,
}
