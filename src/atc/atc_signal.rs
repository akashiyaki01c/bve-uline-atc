/// ATC信号を表す
#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Debug)]
pub enum AtcSignal {
    /// 02信号(絶対停止)
    Signal02 = 0,
    /// 01信号(許容停止)
    Signal01 = 1,
    /// 15信号
    Signal15 = 2,
    /// 25信号
    Signal25 = 3,
    /// 45信号
    Signal45 = 4,
    /// 60信号
    Signal60 = 5,
    /// 75信号
    Signal75 = 6,
    /// 90信号
    Signal90 = 7,
    /// 入換15信号
    Irekae15 = 8,
    /// 入換25信号
    Irekae25 = 9,
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
            Self::Irekae15 => 15,
            Self::Irekae25 => 25,
        }
    }
}
impl Default for AtcSignal {
    fn default() -> Self {
        Self::Signal02
    }
}