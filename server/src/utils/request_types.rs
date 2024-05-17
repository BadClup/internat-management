use serde::{Deserialize, Serialize};

pub const DEFAULT_TAKE: u32 = 10;
pub const DEFAULT_SKIP: u32 = 0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TakeSkip {
    pub take: Option<u32>,
    pub skip: Option<u32>,
}
