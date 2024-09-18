use std::u16;

use crate::MeowPosition;

impl MeowPosition {
    pub fn ret(&self) -> (u16, u16) {
        return (self.x, self.y);
    }
}
