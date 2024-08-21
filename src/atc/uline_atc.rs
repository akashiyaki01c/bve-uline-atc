use ::bveats_rs::*;
use crate::{atc::{atc_signal::*, auto_brake::{elapse_hisetsu_brake, elapse_irekae_brake}}, tims::TIMS};

use super::auto_brake::elapse_atc_brake;

/// TIMS 力行表示パターン
const POWER_PATTERN: [[i32; 8]; 8] = [
    [1, 1, 1, 0, 0, 0, 0, 0], // 抑速3ノッチ
    [0, 1, 1, 0, 0, 0, 0, 0], // 抑速2ノッチ
    [0, 0, 1, 0, 0, 0, 0, 0], // 抑速1ノッチ
    [0, 0, 0, 1, 0, 0, 0, 0], // 切
    [0, 0, 0, 0, 1, 0, 0, 0], // 力行1ノッチ
    [0, 0, 0, 0, 1, 1, 0, 0], // 力行2ノッチ
    [0, 0, 0, 0, 1, 1, 1, 0], // 力行3ノッチ
    [0, 0, 0, 0, 1, 1, 1, 1], // 力行4ノッチ
];
/// TIMS ブレーキ表示パターン
const BRAKE_PATTERN: [[i32; 9]; 9] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0], // 弛め
    [0, 1, 0, 0, 0, 0, 0, 0, 0], // ブレーキ1ノッチ
    [0, 1, 1, 0, 0, 0, 0, 0, 0], // ブレーキ2ノッチ
    [0, 1, 1, 1, 0, 0, 0, 0, 0], // ブレーキ3ノッチ
    [0, 1, 1, 1, 1, 0, 0, 0, 0], // ブレーキ4ノッチ
    [0, 1, 1, 1, 1, 1, 0, 0, 0], // ブレーキ5ノッチ
    [0, 1, 1, 1, 1, 1, 1, 0, 0], // ブレーキ6ノッチ
    [0, 1, 1, 1, 1, 1, 1, 1, 0], // ブレーキ7ノッチ
    [0, 1, 1, 1, 1, 1, 1, 1, 1], // 非常ブレーキ
];
/// panelのサイズ
const ELAPSE_PANEL_SIZE: usize = 256;

/// 現在のATCブレーキ種別
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum AtcBrakeStatus {
    /// ATCブレーキ制御なし
    Passing,
    /// ATC通常ブレーキ中
    FullBraking,
    /// ATC緩和ブレーキ中 (ATCブレーキ設定)
    HalfBraking(i32),
    /// ATC非常ブレーキ中
    EmergencyBraking,
}
impl Default for AtcBrakeStatus {
    fn default() -> Self {
        Self::Passing
    }
}

/// 現在のATC種別
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum AtcStatus {
    /// ATO制御
    ATO,
    /// ATC制御
    ATC,
    /// 入換
    Irekae,
    /// 非設
    Hisetsu,
}
impl Default for AtcStatus {
    fn default() -> Self {
        Self::ATC
    }
}
impl AtcStatus {
    /// 運転切り替えスイッチを右に回した時のステータス
    pub fn get_right_status(&self) -> AtcStatus {
        match self {
            AtcStatus::ATO => AtcStatus::ATO,
            AtcStatus::ATC => AtcStatus::ATO,
            AtcStatus::Irekae => AtcStatus::ATC,
            AtcStatus::Hisetsu => AtcStatus::Irekae
        }
    }
    /// 運転切り替えスイッチを左に回した時のステータス
    pub fn get_left_status(&self) -> AtcStatus {
        match self {
            AtcStatus::ATO => AtcStatus::ATC,
            AtcStatus::ATC => AtcStatus::Irekae,
            AtcStatus::Irekae => AtcStatus::Hisetsu,
            AtcStatus::Hisetsu => AtcStatus::Hisetsu
        }
    }
}

/// 非常放送の種類を表す
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum EmgSound {
    /// なし
    None,
    /// 信号待ち
    SignalWait,
    /// 急病人
    EmergencyCase,
    /// 緊急停止
    EmergencyStop,
    /// シート交換
    SeatExchange,
    /// 非常ブレーキ
    EmergencyBrake,
}
impl Default for EmgSound {
    fn default() -> Self {
        Self::None
    }
}

/// 非常放送のKeyDown時の情報を保持する列挙体
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum EmgSoundKeyDown {
    None,
    /// 信号待ち
    H(i32),
    /// 急病人
    I(i32),
    /// 緊急停止
    J(i32),
    /// シート交換
    K(i32),
    /// 非常ブレーキ
    L(i32),
}
impl Default for EmgSoundKeyDown {
    fn default() -> Self {
        Self::None
    }
}

pub struct ULineATC {
    /// 車両諸元
    pub vehicle_spec: AtsVehicleSpec,
    /// 信号が変化した直後か
    pub is_changing_signal: bool,
    
    /// 入力されている力行ノッチ
    pub man_power: i32,
    /// 入力されているブレーキノッチ
    pub man_brake: i32,
    /// 入力されているレバーサ
    pub man_reverser: i32,

    /// 現在の信号
    pub now_signal: AtcSignal,
    /// ATCブレーキの種別
    pub atc_brake_status: AtcBrakeStatus,
    /// 非常運転が有効になっているか
    pub enable_02hijo_unten: bool,
    /// 確認運転が有効になっているか
    pub enable_01kakunin_unten: bool,

    // Natives
    pub time: i32,
    pub speed: f32,

    /// TIMS
    tims: TIMS,
    /// TIMS用のパネル配列 (ラグ対応用)
    tims_panel: Box<[i32; ELAPSE_PANEL_SIZE]>,

    /// ATCの状態
    pub atc_status: AtcStatus,
    
    /// 非常放送
    emg_sound: EmgSound,
    emg_sound_keydown: EmgSoundKeyDown,
    is_emg_brake_sound: bool,

    /// 現在定速制御中か
    pub is_constant_control: bool,
    /// 現在抑速制御中か
    pub is_holding_control: bool,
}

impl ULineATC {
    fn elapse_display(&mut self, _state: AtsVehicleState, _panel: &mut [i32], handles: &AtsHandles) {
        for i in 31..=38 { self.tims_panel[i] = 0; }
        match self.now_signal {
            AtcSignal::Signal02 => self.tims_panel[31] = 1,
            AtcSignal::Signal01 => self.tims_panel[32] = 1,
            AtcSignal::Signal15 => self.tims_panel[33] = 1,
            AtcSignal::Signal25 => self.tims_panel[34] = 1,
            AtcSignal::Signal45 => self.tims_panel[35] = 1,
            AtcSignal::Signal60 => self.tims_panel[36] = 1,
            AtcSignal::Signal75 => self.tims_panel[37] = 1,
            AtcSignal::Signal90 => self.tims_panel[38] = 1,
            _ => {}
        }
        for i in 0..8 {
            self.tims_panel[11+i] = POWER_PATTERN[(handles.power as usize)+3][i];
        }
        for i in 0..9 {
            self.tims_panel[21+i] = BRAKE_PATTERN[handles.brake as usize][i];
        }
        self.tims_panel[40] = if self.enable_02hijo_unten { 1 } else { 0 };
        self.tims_panel[41] = if self.enable_01kakunin_unten { 1 } else { 0 };
        self.tims_panel[19] = self.is_constant_control as i32;
    }
    fn elapse_emg_sound(&mut self, sound: &mut [i32]) {
        for i in 101..=105 { sound[i] = AtsSound::Continue as i32; }
        match self.emg_sound {
            EmgSound::SignalWait => sound[101] = AtsSound::Play as i32,
            EmgSound::EmergencyCase => sound[102] = AtsSound::Play as i32,
            EmgSound::EmergencyStop => sound[103] = AtsSound::Play as i32,
            EmgSound::SeatExchange => sound[104] = AtsSound::Play as i32,
            EmgSound::EmergencyBrake => sound[105] = AtsSound::Play as i32,
            _ => {}
        }
        if self.is_emg_brake_sound {
            sound[106] = AtsSound::Play as i32;
            self.is_emg_brake_sound = false;
        } else {
            sound[106] = AtsSound::Continue as i32;
        }
    }
    fn show_atc_status(&mut self, _panel: &mut [i32]) {
        for i in 42..=45 { self.tims_panel[i] = 0; }
        match self.atc_status {
            AtcStatus::Hisetsu => self.tims_panel[42] = 1,
            AtcStatus::Irekae => self.tims_panel[43] = 1,
            AtcStatus::ATC => self.tims_panel[44] = 1,
            AtcStatus::ATO => self.tims_panel[45] = 1,
        }
    }
}

impl BveAts for ULineATC {

    fn load(&mut self) {
        println!("Load");
        self.tims.load();
    }
    fn dispose(&mut self) {
        println!("Dispose");
        self.tims.dispose();
    }
    fn get_plugin_version(&mut self) -> i32 { 
        println!("GetPluginVersion"); 
        ATS_VERSION 
    }
    fn set_vehicle_spec(&mut self, spec: AtsVehicleSpec) {
        println!("SetVehicleSpec: {:?}", spec);
        self.vehicle_spec = spec;
        self.tims.set_vehicle_spec(spec);
    }
    fn initialize(&mut self, handle: AtsInit) {
        self.tims.initialize(handle);
    }

    fn elapse(&mut self, state: AtsVehicleState, panel: &mut [i32], sound: &mut [i32]) -> AtsHandles {
        self.time = state.time;
        self.speed = state.speed;
        self.show_atc_status(panel);
        self.elapse_emg_sound(sound);
        self.tims.elapse(state, panel, sound);

        let handles = match self.atc_status {
            AtcStatus::ATO => elapse_atc_brake(self, state, sound),
            AtcStatus::ATC => elapse_atc_brake(self, state, sound),
            AtcStatus::Irekae => elapse_irekae_brake(self, state, sound),
            AtcStatus::Hisetsu => elapse_hisetsu_brake(self, state, sound)
        };
        self.elapse_display(state, panel, &handles);

        for i in 0..(panel.len().min(self.tims_panel.len())) {
            if i < ELAPSE_PANEL_SIZE {
                panel[i] = self.tims_panel[i];
            }
        }
        handles
    }
    fn set_power(&mut self, notch: i32) {
        println!("SetPower: {:?}", notch);
        self.is_constant_control = self.man_power == 4 && notch == 3 && self.speed >= 15.0;
        self.is_holding_control = self.man_power == -2 && notch == -1 && self.speed >= 25.0;
        self.man_power = notch;
        self.tims.set_power(notch);
    }
    fn set_brake(&mut self, notch: i32) {
        println!("SetBrake: {:?}", notch);
        self.man_brake = notch;
        if notch == self.vehicle_spec.brake_notches + 1 && self.speed > 5.0 {
            self.is_emg_brake_sound = true;
        }

        self.tims.set_brake(notch);
    }
    fn set_reverser(&mut self, notch: i32) {
        println!("SetReverser: {:?}", notch);
        self.man_reverser = notch;
        self.tims.set_reverser(notch);
    }
    fn key_down(&mut self, key: AtsKey) {
        println!("KeyDown: {:?}", key);
        match key {
            AtsKey::D => { // 2 非常運転
                self.enable_01kakunin_unten = false;
                self.enable_02hijo_unten = true;
            }
            AtsKey::E => { // 3 確認運転
                self.enable_01kakunin_unten = true;
                self.enable_02hijo_unten = false;
            }
            AtsKey::C1 => { // PageUp 運転切換スイッチ左
                self.atc_status = self.atc_status.get_left_status();
            }
            AtsKey::C2 => { // PageDown 運転切換スイッチ右
                self.atc_status = self.atc_status.get_right_status();
            }
            AtsKey::H => { // 6 非常放送 信号待ち
                if let EmgSoundKeyDown::None = self.emg_sound_keydown {
                    self.emg_sound_keydown = EmgSoundKeyDown::H(self.time);
                }
                match self.emg_sound_keydown {
                    EmgSoundKeyDown::H(time) => {
                        if (self.time - time) > 1000 {
                            self.emg_sound = EmgSound::SignalWait;
                        }
                    }
                    _ => {
                        self.emg_sound_keydown = EmgSoundKeyDown::H(self.time);
                    }
                }
            }
            AtsKey::I => { // 7 非常放送 急病人対応
                if let EmgSoundKeyDown::None = self.emg_sound_keydown {
                    self.emg_sound_keydown = EmgSoundKeyDown::I(self.time);
                }
                match self.emg_sound_keydown {
                    EmgSoundKeyDown::I(time) => {
                        if (self.time - time) > 1000 {
                            self.emg_sound = EmgSound::EmergencyCase;
                        }
                    }
                    _ => {
                        self.emg_sound_keydown = EmgSoundKeyDown::I(self.time);
                    }
                }
            }
            AtsKey::J => { // 8 非常放送 緊急停止
                if let EmgSoundKeyDown::None = self.emg_sound_keydown {
                    self.emg_sound_keydown = EmgSoundKeyDown::J(self.time);
                }
                match self.emg_sound_keydown {
                    EmgSoundKeyDown::J(time) => {
                        if (self.time - time) > 1000 {
                            self.emg_sound = EmgSound::EmergencyStop;
                        }
                    }
                    _ => {
                        self.emg_sound_keydown = EmgSoundKeyDown::J(self.time);
                    }
                }
            }
            AtsKey::K => { // 9 非常放送 シート交換
                if let EmgSoundKeyDown::None = self.emg_sound_keydown {
                    self.emg_sound_keydown = EmgSoundKeyDown::K(self.time);
                }
                match self.emg_sound_keydown {
                    EmgSoundKeyDown::K(time) => {
                        if (self.time - time) > 1000 {
                            self.emg_sound = EmgSound::SeatExchange;
                        }
                    }
                    _ => {
                        self.emg_sound_keydown = EmgSoundKeyDown::K(self.time);
                    }
                }
            }
            AtsKey::L => { // 0 非常放送 非常ブレーキ
                if let EmgSoundKeyDown::None = self.emg_sound_keydown {
                    self.emg_sound_keydown = EmgSoundKeyDown::L(self.time);
                }
                match self.emg_sound_keydown {
                    EmgSoundKeyDown::L(time) => {
                        if (self.time - time) > 1000 {
                            self.emg_sound = EmgSound::EmergencyBrake;
                        }
                    }
                    _ => {
                        self.emg_sound_keydown = EmgSoundKeyDown::L(self.time);
                    }
                }
            }
            _ => {}
        }
        self.tims.key_down(key);
    }
    fn key_up(&mut self, key: AtsKey) {
        println!("KeyUp: {:?}", key);
        match key {
            AtsKey::H | AtsKey::I | AtsKey::J | AtsKey::K | AtsKey::L => {
                self.emg_sound_keydown = EmgSoundKeyDown::None
            }
            _ => {}
        }
        self.tims.key_up(key);
    }
    fn horn_blow(&mut self, horn_type: AtsHorn) {
        println!("HornBlow: {:?}", horn_type);
        self.tims.horn_blow(horn_type);
    }
    fn door_open(&mut self) {
        println!("DoorOpen");
        self.tims.door_open();
    }
    fn door_close(&mut self) {
        println!("DoorClose");
        self.tims.door_close();
    }
    fn set_signal(&mut self, signal: i32) {
        println!("SetSignal: {:?}", signal);
        if 0 <= signal && signal <= 7 {
            self.now_signal = unsafe { std::mem::transmute(signal as u8) };
            self.is_changing_signal = true;
            self.tims.set_signal(signal);
        }
    }
    fn set_beacon_data(&mut self, data: AtsBeaconData) {
        println!("SetBeaconData: {:?}", data);
        self.tims.set_beacon_data(data);
    }
}

impl Default for ULineATC {
    fn default() -> Self {
        Self { 
            man_power: 0, 
            man_brake: 0, 
            man_reverser: 0, 
            now_signal: AtcSignal::default(), 
            vehicle_spec: AtsVehicleSpec::default(), 
            is_changing_signal: false,
            atc_brake_status: AtcBrakeStatus::Passing,
            tims: TIMS::default(),
            atc_status: AtcStatus::default(),
            enable_01kakunin_unten: false,
            enable_02hijo_unten: false,
            emg_sound: EmgSound::default(),
            emg_sound_keydown: EmgSoundKeyDown::default(),
            time: 0,
            speed: 0.0,
            is_emg_brake_sound: false,
            is_constant_control: false,
            is_holding_control: false,
            tims_panel: Box::new([0; ELAPSE_PANEL_SIZE]),
        }
    }
}