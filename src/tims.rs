use bveats_rs::{AtsBeaconData, AtsHorn, AtsInit, AtsKey, AtsVehicleSpec, AtsVehicleState};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
enum ULineStation {
    None = 0,
    S01Tanigami = 1,
    S02ShinKobe = 2,
    S03Sannomiya = 3,
    S04Kenchomae = 4,
    S05Okurayama = 5,
    S06MinatogawaKoen = 6,
    S07Kamisawa = 7,
    S08Nagata = 8,
    S09ShinNagata = 9,
    S10Itayado = 10,
    S11Myohoji = 11,
    S12Myodani = 12,
    S13SogoundoKoen = 13,
    S14Gakuentoshi = 14,
    S15Ikawadani = 15,
    S16SeishinMinami = 16,
    S17SeishinChuo = 17,
}
impl Default for ULineStation {
	fn default() -> Self {
		Self::None
	}
}
impl ULineStation {
	pub fn to_i32(self) -> i32 {
		unsafe { std::mem::transmute(self) }
	}
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
enum ULineTrainType {
    None = 0,
    Local = 1,
    OutOfService = 2,
    TestRun = 3,
    Rapid1 = 4,
    Rapid2 = 5,
}
impl ULineTrainType {
	pub fn to_i32(self) -> i32 {
		unsafe { std::mem::transmute(self) }
	}
}
impl Default for ULineTrainType {
	fn default() -> Self {
		Self::None
	}
}

/// TIMSを表す
#[derive(Default)]
pub struct TIMS {
	/// TIMS 始発駅
    start_station: ULineStation,
    /// TIMS 終着駅
    destination: ULineStation,
    /// TIMS 列車種別
    train_type: ULineTrainType,
    /// TIMS 列車番号
    operation_number: i32,
}

impl TIMS {
	pub(super) fn load(&mut self) {
    }

    pub(super) fn dispose(&mut self) {
    }

    pub(super) fn set_vehicle_spec(&mut self, _spec: AtsVehicleSpec) {
    }

    pub(super) fn initialize(&mut self, _handle: AtsInit) {
    }

    pub(super) fn elapse(&mut self, _state: AtsVehicleState, _panel: &mut [i32], _sound: &mut [i32]) {
		let total_second = _state.time / 1000;
		let hours = total_second / 60 / 60;
		let minutes = total_second / 60 % 60;
		let seconds = total_second % 60;

		_panel[101] = self.operation_number / 10;
		_panel[102] = self.operation_number % 10;
		_panel[103] = self.train_type.to_i32();
		_panel[104] = self.destination.to_i32();
		// _panel[105] = 
		// _panel[106] =
		_panel[107] = hours / 10;
		_panel[108] = hours % 10;
		_panel[109] = minutes / 10;
		_panel[110] = minutes % 10;
		_panel[111] = seconds / 10;
		_panel[112] = seconds % 10;
	}
	pub(super) fn set_power(&mut self, _notch: i32) {
    }

    pub(super) fn set_brake(&mut self, _notch: i32) {
    }

    pub(super) fn set_reverser(&mut self, _notch: i32) {
    }

    pub(super) fn key_down(&mut self, _key: AtsKey) {
    }

    pub(super) fn key_up(&mut self, _key: AtsKey) {
    }

    pub(super) fn horn_blow(&mut self, _horn_type: AtsHorn) {
    }

    pub(super) fn door_open(&mut self) {
    }

    pub(super) fn door_close(&mut self) {
    }

    pub(super) fn set_signal(&mut self, _signal: i32) {
    }
	pub(super) fn set_beacon_data(&mut self, data: AtsBeaconData) {
		match data.beacon_type {
            11 => { // 始発駅設定
                if 0 <= data.optional && data.optional <= 17 {
                    self.start_station = unsafe { std::mem::transmute(data.optional) };
                }
            },
            12 => { // 行先設定
                if 0 <= data.optional && data.optional <= 17 {
                    self.destination = unsafe { std::mem::transmute(data.optional) };
                }
            },
            13 => { // 種別設定
                if 0 <= data.optional && data.optional <= 5 {
                    self.train_type = unsafe { std::mem::transmute(data.optional) };
                }
            },
            14 => { // 運番設定
                if 0 <= data.optional && data.optional <= 99 {
                    self.operation_number = data.optional;
                }
            },
            _ => println!("[ATS_WARN]: 定義されていない地上子番号です。")
        }
	}
}