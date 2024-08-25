#![allow(unused)]
use std::os::raw::*;

/// GetPluginVersion() の戻り値を表す。
pub const ATS_VERSION: c_int = 0x00020000;

/// ATS キー コード
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum AtsKey {
    /// S ボタン (デフォルト Space)
    S = 0,
    /// A1 ボタン (デフォルト Insert)
    A1 = 1,
    /// A2 ボタン (デフォルト Delete)
    A2 = 2,
    /// B1 ボタン (デフォルト Home)
    B1 = 3,
    /// B2 ボタン (デフォルト End)
    B2 = 4,
    /// C1 ボタン (デフォルト Page Up)
    C1 = 5,
    /// C2 ボタン (デフォルト Page Down)
    C2 = 6,
    /// D ボタン (デフォルト 2)
    D = 7,
    /// E ボタン (デフォルト 3)
    E = 8,
    /// F ボタン (デフォルト 4)
    F = 9,
    /// G ボタン (デフォルト 5)
    G = 10,
    /// H ボタン (デフォルト 6)
    H = 11,
    /// I ボタン (デフォルト 7)
    I = 12,
    /// J ボタン (デフォルト 8)
    J = 13,
    /// K ボタン (デフォルト 9)
    K = 14,
    /// L ボタン (デフォルト 0)
    L = 15,
    /// 予期しない値
    Unknown = -1,
}
impl From<i32> for AtsKey {
    fn from(value: i32) -> Self {
        unsafe {
            if 0 <= value && value <= 15 {
                std::mem::transmute(value)
            } else {
                AtsKey::Unknown
            }
        }
    }
}

/// ゲーム開始時のブレーキ弁の状態
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum AtsInit {
    /// 抜き取り (通常、保安装置未投入)
    Removed = 2,
    /// 非常位置
    Emg = 1,
    /// 常用位置
    Svc = 0,
    /// 予期しない値
    Unknown = -1,
}
impl From<i32> for AtsInit {
    fn from(value: i32) -> Self {
        unsafe {
            if 0 <= value && value <= 15 {
                std::mem::transmute(value)
            } else {
                AtsInit::Unknown
            }
        }
    }
}

/// サウンドのボリューム
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum AtsSound {
    /// 停止
    Stop = -10000,
    /// 1 回再生
    Play = 1,
    /// 繰り返し再生
    PlayLooping = 0,
    /// 現在の状態を維持する
    Continue = 2,
    /// 予期しない値
    Unknown = -1,
}
impl From<i32> for AtsSound {
    fn from(value: i32) -> Self {
        unsafe {
            if 0 <= value && value <= 15 {
                std::mem::transmute(value)
            } else {
                AtsSound::Unknown
            }
        }
    }
}

/// 警笛のタイプ
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum AtsHorn {
    /// 警笛1
    Primary = 0,
    /// 警笛2
    Secondary = 1,
    /// ミュージックホーン
    Music = 2,
    /// 予期しない値
    Unknown = -1,
}
impl From<i32> for AtsHorn {
    fn from(value: i32) -> Self {
        unsafe {
            if 0 <= value && value <= 15 {
                std::mem::transmute(value)
            } else {
                AtsHorn::Unknown
            }
        }
    }
}

/// 定速制御の状態
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum AtsConstantSpeed {
    /// 現在の状態を維持する
    Continue = 0,
    /// 起動
    Enable = 1,
    /// 停止
    Disable = 2,
    /// 予期しない値
    Unknown = -1,
}
impl From<i32> for AtsConstantSpeed {
    fn from(value: i32) -> Self {
        unsafe {
            if 0 <= value && value <= 15 {
                std::mem::transmute(value)
            } else {
                AtsConstantSpeed::Unknown
            }
        }
    }
}
impl Default for AtsConstantSpeed {
    fn default() -> Self {
        Self::Continue
    }
}

/// 車両諸元
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsVehicleSpec {
    /// ブレーキノッチ数
    pub brake_notches: c_int,
    /// 力行ノッチ数
    pub power_notches: c_int,
    /// ATS確認ノッチ
    pub ats_notch: c_int,
    /// ブレーキ弁 67 度に相当するノッチ
    pub b67_notch: c_int,
    /// 編成両数
    pub cars: c_int,
}

/// 車両の状態量
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsVehicleState {
    /// 列車位置 [m]
    pub location: c_double,
    /// 列車速度 [km/h]
    pub speed: c_float,
    /// 現在時刻 [ms]
    pub time: c_int,
    /// ブレーキシリンダ圧力 [kPa]
    pub bc_pressure: c_float,
    /// 元空気ダメ圧力 [kPa]
    pub mr_pressure: c_float,
    /// 釣り合い空気ダメ圧力 [kPa]
    pub er_pressure: c_float,
    /// ブレーキ管圧力 [kPa]
    pub bp_pressure: c_float,
    /// 直通管圧力 [kPa]
    pub sap_pressure: c_float,
    /// 電流 [A]
    pub current: c_float,
}

/// 車上子で受け取った情報
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsBeaconData {
    /// 地上子種別
    pub beacon_type: c_int,
    /// 対となるセクションの信号
    pub signal: c_int,
    /// 対となるセクションまでの距離 [m]
    pub distance: c_float,
    /// 地上子に設定された任意の値
    pub optional: c_int,
}

/// Bve trainsim に渡すハンドル制御値
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsHandles {
    /// ブレーキノッチ
    pub brake: c_int,
    /// 力行ノッチ
    pub power: c_int,
    /// レバーサー位置
    pub reverser: c_int,
    /// 定速制御の状態
    pub constant_speed: c_int,
}

/// Rust上でのATS実装をサポートするトレイト
pub trait BveAts {
    /// プラグインが読み込まれたときに呼び出される関数
    fn load(&mut self);
    /// プラグインが解放されたときに呼び出される関数
    fn dispose(&mut self);
    /// この ATS プラグインが準じているフォーマットを返す関数
    fn get_plugin_version(&mut self) -> i32 { ATS_VERSION }
    /// 車両読み込み時に呼び出される関数
    fn set_vehicle_spec(&mut self, spec: AtsVehicleSpec);
    /// ゲーム開始 ([開く] または [はじめから] 選択) 時に呼び出される関数
    fn initialize(&mut self, handle: AtsInit);
    /// 1 フレームごとに呼び出される関数です。
    fn elapse(&mut self, state: AtsVehicleState, panel: &mut [i32], sound: &mut [i32]) -> AtsHandles;
    /// 主ハンドルが扱われたときに呼び出される関数
    fn set_power(&mut self, notch: i32);
    /// ブレーキが扱われたときに呼び出される関数
    fn set_brake(&mut self, notch: i32);
    /// レバーサーが扱われたときに呼び出される関数
    fn set_reverser(&mut self, notch: i32);
    /// ATS キーが押されたときに呼び出される関数
    fn key_down(&mut self, key: AtsKey);
    /// ATS キーが離されたときに呼び出される関数
    fn key_up(&mut self, key: AtsKey);
    /// 警笛が扱われたときに呼び出される関数
    fn horn_blow(&mut self, horn_type: AtsHorn);
    /// 客室ドアが開いたときに呼び出される関数
    fn door_open(&mut self);
    /// 客室ドアが閉まったときに呼び出される関数
    fn door_close(&mut self);
    /// 現在の閉そくの信号が変化したときに呼び出される関数
    fn set_signal(&mut self, signal: i32);
    /// 地上子を越えたときに呼び出される関数
    fn set_beacon_data(&mut self, data: AtsBeaconData);
}

/// BveAtsトレイト からネイティブAPIへの変換を行うマクロ
#[macro_export]
macro_rules! ats_main {
    ($t: ty) => {
        use ::std::sync::Mutex;
        use ::std::sync::OnceLock;
        use ::std::os::raw::*;
        use ::bveats_rs::*;
        static ATS: OnceLock<Mutex<$t>> = OnceLock::new();

        #[no_mangle]
        pub unsafe extern "system" fn Load() {
            ATS .get_or_init(|| Mutex::new(<$t>::default()))
                .lock()
                .expect("Mutex error: at Load()")
                .load();
        }

        #[no_mangle]
        pub unsafe extern "system" fn Dispose() {
            ATS .get()
                .expect("OnceLock error: at Dispose()")
                .lock()
                .expect("Mutex error: at Dispose()")
                .dispose()
        }

        #[no_mangle]
        pub unsafe extern "system" fn GetPluginVersion() -> c_int {
            ATS .get()
                .expect("OnceLock error: at GetPluginVersion()")
                .lock()
                .expect("Mutex error: at GetPluginVersion()")
                .get_plugin_version()
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetVehicleSpec(spec: AtsVehicleSpec) {
            ATS .get()
                .expect("OnceLock error: at SetVehicleSpec()")
                .lock()
                .expect("Mutex error: at SetVehicleSpec()")
                .set_vehicle_spec(spec)
        }

        #[no_mangle]
        pub unsafe extern "system" fn Initialize(brake: c_int) {
            ATS .get()
                .expect("OnceLock error: at Initialize()")
                .lock()
                .expect("Mutex error: at Initialize()")
                .initialize(AtsInit::from(brake))
        }

        #[no_mangle]
        pub unsafe extern "system" fn Elapse(state: AtsVehicleState, panel: *mut c_int, sound: *mut c_int) -> AtsHandles {
            ATS .get()
                .expect("OnceLock error: at Elapse()")
                .lock()
                .expect("Mutex error: at Elapse()")
                .elapse(state, 
                    std::slice::from_raw_parts_mut(panel, 256), 
                    std::slice::from_raw_parts_mut(sound, 256))
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetPower(notch: c_int) {
            ATS .get()
                .expect("OnceLock error: at SetPower()")
                .lock()
                .expect("Mutex error: at SetPower()")
                .set_power(notch)
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetBrake(notch: c_int) {
            ATS .get()
                .expect("OnceLock error: at SetBrake()")
                .lock()
                .expect("Mutex error: at SetBrake()")
                .set_brake(notch)
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetReverser(notch: c_int) {
            ATS .get()
                .expect("OnceLock error: at SetReverser()")
                .lock()
                .expect("Mutex error: at SetReverser()")
                .set_reverser(notch)
        }

        #[no_mangle]
        pub unsafe extern "system" fn KeyDown(key: c_int) {
            ATS .get()
                .expect("OnceLock error: at KeyDown()")
                .lock()
                .expect("Mutex error: at KeyDown()")
                .key_down(AtsKey::from(key))
        }

        #[no_mangle]
        pub unsafe extern "system" fn KeyUp(key: c_int) {
            ATS .get()
                .expect("OnceLock error: at KeyUp()")
                .lock()
                .expect("Mutex error: at KeyUp()") 
                .key_up(AtsKey::from(key))
        }

        #[no_mangle]
        pub unsafe extern "system" fn HornBlow(horn: c_int) {
            ATS .get()
                .expect("OnceLock error: at HornBlow()")
                .lock()
                .expect("Mutex error: at HornBlow()")
                .horn_blow(AtsHorn::from(horn))
        }

        #[no_mangle]
        pub unsafe extern "system" fn DoorOpen() {
            ATS .get()
                .expect("OnceLock error: at DoorOpen()")
                .lock()
                .expect("Mutex error: at DoorOpen()")
                .door_open()
        }

        #[no_mangle]
        pub unsafe extern "system" fn DoorClose() {
            ATS .get()
                .expect("OnceLock error: at DoorClose()")
                .lock()
                .expect("Mutex error: at DoorClose()")
                .door_close()
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetSignal(signal: c_int) {
            ATS .get()
                .expect("OnceLock error: at SetSignal()")
                .lock()
                .expect("Mutex error: at SetSignal()")
                .set_signal(signal)
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetBeaconData(data: AtsBeaconData) {
            ATS .get()
                .expect("OnceLock error: at SetBeaconData()")
                .lock()
                .expect("Mutex error: at SetBeaconData()")
                .set_beacon_data(data)
        }
    };
}

/// BveAtsトレイト からネイティブAPIへの変換を行うマクロ
#[macro_export]
macro_rules! ats_main_empty {
    () => {
        use ::std::sync::Mutex;
        use ::std::sync::OnceLock;
        use ::std::os::raw::*;
        use ::bveats_rs::*;

        #[no_mangle]
        pub unsafe extern "system" fn Load() {
        }

        #[no_mangle]
        pub unsafe extern "system" fn Dispose() {
        }

        #[no_mangle]
        pub unsafe extern "system" fn GetPluginVersion() -> c_int {
            ATS_VERSION
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetVehicleSpec(spec: AtsVehicleSpec) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn Initialize(brake: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn Elapse(state: AtsVehicleState, panel: *mut c_int, sound: *mut c_int) -> AtsHandles {
            Default::default()
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetPower(notch: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetBrake(notch: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetReverser(notch: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn KeyDown(key: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn KeyUp(key: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn HornBlow(horn: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn DoorOpen() {
        }

        #[no_mangle]
        pub unsafe extern "system" fn DoorClose() {
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetSignal(signal: c_int) {
        }

        #[no_mangle]
        pub unsafe extern "system" fn SetBeaconData(data: AtsBeaconData) {
        }
    };
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EmptyAts {
    power: i32,
    brake: i32,
    reverser: i32,
}
impl BveAts for EmptyAts {
    fn load(&mut self) {
        eprintln!("[EmptyAts] Load()");
    }

    fn dispose(&mut self) {
        eprintln!("[EmptyAts] Dispose()");
    }

    fn set_vehicle_spec(&mut self, spec: AtsVehicleSpec) {
        eprintln!("[EmptyAts] SetVehicleSpec() spec={:?}", spec);
    }

    fn initialize(&mut self, handle: AtsInit) {
        eprintln!("[EmptyAts] Initialize() handle={:?}", handle);
    }

    fn elapse(&mut self, state: AtsVehicleState, panel: &mut [i32], sound: &mut [i32]) -> AtsHandles {
        eprintln!("[EmptyAts] Elapse() state={:?}", state);
        AtsHandles {
            brake: self.brake,
            power: self.power,
            reverser: self.reverser,
            constant_speed: 0
        }
    }

    fn set_power(&mut self, notch: i32) {
        eprintln!("[EmptyAts] SetPower() handle={:?}", notch);
        self.power = notch;
    }

    fn set_brake(&mut self, notch: i32) {
        eprintln!("[EmptyAts] SetBrake() handle={:?}", notch);
        self.brake = notch;
    }

    fn set_reverser(&mut self, notch: i32) {
        eprintln!("[EmptyAts] SetReverser() handle={:?}", notch);
        self.reverser = notch;
    }

    fn key_down(&mut self, key: AtsKey) {
        eprintln!("[EmptyAts] KeyDown() key={:?}", key);
    }

    fn key_up(&mut self, key: AtsKey) {
        eprintln!("[EmptyAts] KeyUp() key={:?}", key);
    }

    fn horn_blow(&mut self, horn_type: AtsHorn) {
        eprintln!("[EmptyAts] KeyDown() hornType={:?}", horn_type);
    }

    fn door_open(&mut self) {
        eprintln!("[EmptyAts] DoorOpen()");
    }

    fn door_close(&mut self) {
        eprintln!("[EmptyAts] DoorClose()");
    }

    fn set_signal(&mut self, signal: i32) {
        eprintln!("[EmptyAts] SetSignal() signal={}", signal);
    }

    fn set_beacon_data(&mut self, data: AtsBeaconData) {
        eprintln!("[EmptyAts] SetBeaconData() signal={:?}", data);
    }
}