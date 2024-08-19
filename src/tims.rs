use bveats_rs::{AtsBeaconData, AtsHorn, AtsInit, AtsKey, AtsSound, AtsVehicleSpec, AtsVehicleState};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
enum ULineStation {
    None = 0,
    S01Tanigami = 17,
    S02ShinKobe = 16,
    S03Sannomiya = 15,
    S04Kenchomae = 14,
    S05Okurayama = 13,
    S06MinatogawaKoen = 12,
    S07Kamisawa = 11,
    S08Nagata = 10,
    S09ShinNagata = 9,
    S10Itayado = 8,
    S11Myohoji = 7,
    S12Myodani = 6,
    S13SogoundoKoen = 5,
    S14Gakuentoshi = 4,
    S15Ikawadani = 3,
    S16SeishinMinami = 2,
    S17SeishinChuo = 1,
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// TIMSで管理する位置情報の起点を表す
#[derive(Debug)]
pub enum TimsPosition {
    /// 西神線、山手線を表す
    SeishinYamate(f32, f32),
    /// 西神延伸線を表す (prefix=N)
    SeishinEnshin(f32, f32),
    /// 北神線を表す (prefix=N)
    Hokushin(f32, f32)
}
impl Default for TimsPosition {
    fn default() -> Self {
        Self::SeishinYamate(0.0, 0.0)
    }
}
impl TimsPosition {
    /// TIMS画面上の距離を取得する関数
    pub fn get_tims_distance(&self, distance: f32, is_negative: bool) -> f32 {
        match self {
            Self::SeishinYamate(bve_distance, origin) => {
                if is_negative {
                    (origin - bve_distance) - distance
                } else {
                    (origin - bve_distance) + distance
                }
            }
            Self::SeishinEnshin(bve_distance, origin) => {
                if is_negative {
                    (origin - bve_distance) - distance
                } else {
                    (origin - bve_distance) + distance
                }
            }
            Self::Hokushin(bve_distance, origin) => {
                if is_negative {
                    (origin - bve_distance) - distance
                } else {
                    (origin - bve_distance) + distance
                }
            }
        }
    }
    /// TIMS画面上の距離につく接頭辞インデックス
    pub fn get_tims_distance_prefix(&self) -> i32 {
        match self {
            Self::SeishinYamate(_, _) => 0,
            Self::SeishinEnshin(_, _) => 1,
            Self::Hokushin(_, _) => 2,
        }
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
    /// TIMS位置情報
    position: TimsPosition,
    /// TIMS 位置情報を減算していくか
    is_position_negative: bool,
    
    /// BVE上での距離
    bve_distance: f64,

    /// 前回、回送放送が流れた時刻
    out_of_service_sound_time: i32,
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
        self.bve_distance = _state.location;
        self.elapse_out_of_service_sound(_state, _sound);

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

        _panel[116] = self.position.get_tims_distance_prefix();
        let distance = self.position.get_tims_distance(_state.location as f32, self.is_position_negative).abs() as i32;
        _panel[117] = distance / 10000 % 10;
        _panel[118] = distance / 1000 % 10;
        _panel[119] = distance / 100 % 10;
        _panel[120] = distance / 10 % 10;
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
            15 => { // TIMS 距離程プレフィックスの設定
                let (bve_distance, origin) = match self.position {
                    TimsPosition::SeishinYamate(bve_distance, origin) => (bve_distance, origin),
                    TimsPosition::SeishinEnshin(bve_distance, origin) => (bve_distance, origin),
                    TimsPosition::Hokushin(bve_distance, origin) => (bve_distance, origin),
                };
                match data.optional {
                    0 => self.position = TimsPosition::SeishinYamate(bve_distance, origin),
                    1 => self.position = TimsPosition::SeishinEnshin(bve_distance, origin),
                    2 => self.position = TimsPosition::Hokushin(bve_distance, origin),
                    _ => {}
                }
            },
            16 => { // TIMS 距離原点の設定
                self.position = match self.position {
                    TimsPosition::SeishinYamate(_, _) => TimsPosition::SeishinYamate(self.bve_distance as f32, data.optional as f32),
                    TimsPosition::SeishinEnshin(_, _) => TimsPosition::SeishinEnshin(self.bve_distance as f32, data.optional as f32),
                    TimsPosition::Hokushin(_, _) => TimsPosition::Hokushin(self.bve_distance as f32, data.optional as f32),
                };
            },
            17 => { // TIMS 距離加減算の設定
                match data.optional {
                    0 => self.is_position_negative = false,
                    _ => self.is_position_negative = true,
                }
            },
            _ => println!("[ATS_WARN]: 定義されていない地上子番号です。")
        }
	}
}
impl TIMS {
    fn elapse_out_of_service_sound(&mut self, _state: AtsVehicleState, sound: &mut [i32]) {
        if !(self.train_type == ULineTrainType::OutOfService) && !(self.train_type == ULineTrainType::TestRun) {
            return
        }
        if _state.speed > 5.0 {
            return
        }

        if self.out_of_service_sound_time + 10000 < _state.time {
            sound[100] = AtsSound::Play as i32;
            self.out_of_service_sound_time = _state.time;
        } else {
            sound[100] = AtsSound::Continue as i32;
        }
    }
}