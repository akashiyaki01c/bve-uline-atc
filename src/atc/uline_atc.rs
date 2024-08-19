use ::bveats_rs::*;
use crate::{atc::atc_signal::*, tims::TIMS};

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

/// 現在のATCブレーキ種別
#[allow(dead_code)]
#[derive(PartialEq)]
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
#[derive(PartialEq)]
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
#[derive(PartialEq)]
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

    /// TIMS
    tims: TIMS,
    /// ATCの状態
    pub atc_status: AtcStatus,
    /// 非常放送
    emg_sound: EmgSound
}

impl ULineATC {
    fn elapse_display(&mut self, _state: AtsVehicleState, panel: &mut [i32], handles: &AtsHandles) {
        for i in 31..=38 { panel[i] = 0; }
        match self.now_signal {
            AtcSignal::Signal02 => panel[31] = 1,
            AtcSignal::Signal01 => panel[32] = 1,
            AtcSignal::Signal15 => panel[33] = 1,
            AtcSignal::Signal25 => panel[34] = 1,
            AtcSignal::Signal45 => panel[35] = 1,
            AtcSignal::Signal60 => panel[36] = 1,
            AtcSignal::Signal75 => panel[37] = 1,
            AtcSignal::Signal90 => panel[38] = 1,
        }
        for i in 0..8 {
            panel[11+i] = POWER_PATTERN[(handles.power as usize)+3][i];
        }
        for i in 0..9 {
            panel[21+i] = BRAKE_PATTERN[handles.brake as usize][i];
        }
    }
    fn elapse_emg_sound(&self, sound: &mut [i32]) {
        for i in 101..=105 { sound[i] = AtsSound::Continue as i32; }
        match self.emg_sound {
            EmgSound::SignalWait => sound[101] = AtsSound::Play as i32,
            EmgSound::EmergencyCase => sound[102] = AtsSound::Play as i32,
            EmgSound::EmergencyStop => sound[103] = AtsSound::Play as i32,
            EmgSound::SeatExchange => sound[104] = AtsSound::Play as i32,
            EmgSound::EmergencyBrake => sound[105] = AtsSound::Play as i32,
            _ => {}
        }
    }
    fn show_atc_status(&mut self, panel: &mut [i32]) {
        for i in 42..=45 { panel[i] = 0; }
        match self.atc_status {
            AtcStatus::Hisetsu => panel[42] = 1,
            AtcStatus::Irekae => panel[43] = 1,
            AtcStatus::ATC => panel[44] = 1,
            AtcStatus::ATO => panel[45] = 1,
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
        self.show_atc_status(panel);
        self.elapse_emg_sound(sound);
        self.tims.elapse(state, panel, sound);

        let handles = elapse_atc_brake(self, state, sound);
        self.elapse_display(state, panel, &handles);
        handles
    }
    fn set_power(&mut self, notch: i32) {
        println!("SetPower: {:?}", notch);
        self.man_power = notch;
        self.tims.set_power(notch);
    }
    fn set_brake(&mut self, notch: i32) {
        println!("SetBrake: {:?}", notch);
        self.man_brake = notch;
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
            _ => {}
        }
        self.tims.key_down(key);
    }
    fn key_up(&mut self, key: AtsKey) {
        println!("KeyUp: {:?}", key);
        match key {
            AtsKey::H => { // 6 非常放送 信号待ち
                self.emg_sound = EmgSound::SignalWait;
            }
            AtsKey::I => { // 7 非常放送 急病人対応
                self.emg_sound = EmgSound::EmergencyCase;
            }
            AtsKey::J => { // 8 非常放送 緊急停止
                self.emg_sound = EmgSound::EmergencyStop;
            }
            AtsKey::K => { // 9 非常放送 シート交換
                self.emg_sound = EmgSound::SeatExchange;
            }
            AtsKey::L => { // 0 非常放送 非常ブレーキ
                self.emg_sound = EmgSound::EmergencyBrake;
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
        self.now_signal = unsafe { std::mem::transmute(signal as u8) };
        self.is_changing_signal = true;
        self.tims.set_signal(signal);
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
            emg_sound: EmgSound::default()
        }
    }
}