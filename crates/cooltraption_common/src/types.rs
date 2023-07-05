use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TimePoint(u128);
impl TimePoint {
    pub fn from_millis(millis: u128) -> Self {
        Self(millis)
    }

    pub fn millis(&self) -> u128 {
        self.0
    }
}
