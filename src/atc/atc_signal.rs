/// ATC信号を表す
#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum AtcSignal {
    Signal02 = 0,
    Signal01 = 1,
    Signal15 = 2,
    Signal25 = 3,
    Signal45 = 4,
    Signal60 = 5,
    Signal75 = 6,
    Signal90 = 7,
}
impl AtcSignal {
    pub fn getSpeed(&self) -> i32 {
        match self {
            Self::Signal02 => 0,
            Self::Signal01 => 0,
            Self::Signal15 => 15,
            Self::Signal25 => 25,
            Self::Signal45 => 45,
            Self::Signal60 => 60,
            Self::Signal75 => 75,
            Self::Signal90 => 90,
        }
    }
}
impl Default for AtcSignal {
    fn default() -> Self {
        Self::Signal02
    }
}