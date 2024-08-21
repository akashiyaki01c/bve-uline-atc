#![allow(unused)]

/// GetPluginVersion() の戻り値を表す。
pub const ATS_VERSION: i32 = 0x00020000;

/// ATS キー コード
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
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
    L = 15
}

/// ゲーム開始時のブレーキ弁の状態
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum AtsInit {
    /// 抜き取り (通常、保安装置未投入)
    Removed = 2,
    /// 非常位置
    Emg = 1,
    /// 常用位置
    Svc = 0,
}

/// サウンドのボリューム
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum AtsSound {
    /// 停止
    Stop = -10000,
    /// 1 回再生
    Play = 1,
    /// 繰り返し再生
    PlayLooping = 0,
    /// 現在の状態を維持する
    Continue = 2,
}

/// 警笛のタイプ
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum AtsHorn {
    /// 警笛1
    Primary = 0,
    /// 警笛2
    Secondary = 1,
    /// ミュージックホーン
    Music = 2,
}

/// 定速制御の状態
#[repr(i32)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum AtsConstantSpeed {
    /// 現在の状態を維持する
    Continue = 0,
    /// 起動
    Enable = 1,
    /// 停止
    Disable = 2,
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
    pub brake_notches: i32,
    /// 力行ノッチ数
    pub power_notches: i32,
    /// ATS確認ノッチ
    pub ats_notch: i32,
    /// ブレーキ弁 67 度に相当するノッチ
    pub b67_notch: i32,
    /// 編成両数
    pub cars: i32,
}

/// 車両の状態量
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsVehicleState {
    /// 列車位置 [m]
    pub location: f64,
    /// 列車速度 [km/h]
    pub speed: f32,
    /// 現在時刻 [ms]
    pub time: i32,
    /// ブレーキシリンダ圧力 [kPa]
    pub bc_pressure: f32,
    /// 元空気ダメ圧力 [kPa]
    pub mr_pressure: f32,
    /// 釣り合い空気ダメ圧力 [kPa]
    pub er_pressure: f32,
    /// ブレーキ管圧力 [kPa]
    pub bp_pressure: f32,
    /// 直通管圧力 [kPa]
    pub sap_pressure: f32,
    /// 電流 [A]
    pub current: i32,
}

/// 車上子で受け取った情報
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsBeaconData {
    /// 地上子種別
    pub beacon_type: i32,
    /// 対となるセクションの信号
    pub signal: i32,
    /// 対となるセクションまでの距離 [m]
    pub distance: f32,
    /// 地上子に設定された任意の値
    pub optional: i32,
}

/// Bve trainsim に渡すハンドル制御値
#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsHandles {
    /// ブレーキノッチ
    pub brake: i32,
    /// 力行ノッチ
    pub power: i32,
    /// レバーサー位置
    pub reverser: i32,
    /// 定速制御の状態
    pub constant_speed: AtsConstantSpeed,
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
        use ::bveats_rs::*;
        static ATS: OnceLock<Mutex<$t>> = OnceLock::new();

        #[no_mangle]
        pub unsafe extern "C" fn Load() {
            ATS .get_or_init(|| Mutex::new(<$t>::default()))
                .lock()
                .expect("Mutex error: at Load()")
                .load();
        }

        #[no_mangle]
        pub unsafe extern "C" fn Dispose() {
            ATS .get()
                .expect("OnceLock error: at Dispose()")
                .lock()
                .expect("Mutex error: at Dispose()")
                .dispose()
        }

        #[no_mangle]
        pub unsafe extern "C" fn GetPluginVersion() -> i32 {
            ATS .get()
                .expect("OnceLock error: at GetPluginVersion()")
                .lock()
                .expect("Mutex error: at GetPluginVersion()")
                .get_plugin_version()
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetVehicleSpec(spec: AtsVehicleSpec) {
            ATS .get()
                .expect("OnceLock error: at SetVehicleSpec()")
                .lock()
                .expect("Mutex error: at SetVehicleSpec()")
                .set_vehicle_spec(spec)
        }

        #[no_mangle]
        pub unsafe extern "C" fn Initialize(brake: i32) {
            println!("{}", brake);
            ATS .get()
                .expect("OnceLock error: at Initialize()")
                .lock()
                .expect("Mutex error: at Initialize()")
                .initialize(std::mem::transmute(brake))
        }

        #[no_mangle]
        pub unsafe extern "C" fn Elapse(state: AtsVehicleState, panel: *mut i32, sound: *mut i32) -> AtsHandles {
            ATS .get()
                .expect("OnceLock error: at Elapse()")
                .lock()
                .expect("Mutex error: at Elapse()")
                .elapse(state, 
                    std::slice::from_raw_parts_mut(panel, 256), 
                    std::slice::from_raw_parts_mut(sound, 256))
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetPower(notch: i32) {
            ATS .get()
                .expect("OnceLock error: at SetPower()")
                .lock()
                .expect("Mutex error: at SetPower()")
                .set_power(notch)
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetBrake(notch: i32) {
            ATS .get()
                .expect("OnceLock error: at SetBrake()")
                .lock()
                .expect("Mutex error: at SetBrake()")
                .set_brake(notch)
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetReverser(notch: i32) {
            ATS .get()
                .expect("OnceLock error: at SetReverser()")
                .lock()
                .expect("Mutex error: at SetReverser()")
                .set_reverser(notch)
        }

        #[no_mangle]
        pub unsafe extern "C" fn KeyDown(key: i32) {
            ATS .get()
                .expect("OnceLock error: at KeyDown()")
                .lock()
                .expect("Mutex error: at KeyDown()")
                .key_down(std::mem::transmute(key))
        }

        #[no_mangle]
        pub unsafe extern "C" fn KeyUp(key: i32) {
            ATS .get()
                .expect("OnceLock error: at KeyUp()")
                .lock()
                .expect("Mutex error: at KeyUp()")
                .key_up(std::mem::transmute(key))
        }

        #[no_mangle]
        pub unsafe extern "C" fn HornBlow(horn: i32) {
            ATS .get()
                .expect("OnceLock error: at HornBlow()")
                .lock()
                .expect("Mutex error: at HornBlow()")
                .key_up(std::mem::transmute(horn))
        }

        #[no_mangle]
        pub unsafe extern "C" fn DoorOpen() {
            ATS .get()
                .expect("OnceLock error: at DoorOpen()")
                .lock()
                .expect("Mutex error: at DoorOpen()")
                .door_open()
        }

        #[no_mangle]
        pub unsafe extern "C" fn DoorClose() {
            ATS .get()
                .expect("OnceLock error: at DoorClose()")
                .lock()
                .expect("Mutex error: at DoorClose()")
                .door_close()
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetSignal(signal: i32) {
            ATS .get()
                .expect("OnceLock error: at SetSignal()")
                .lock()
                .expect("Mutex error: at SetSignal()")
                .set_signal(signal)
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetBeaconData(data: AtsBeaconData) {
            ATS .get()
                .expect("OnceLock error: at SetBeaconData()")
                .lock()
                .expect("Mutex error: at SetBeaconData()")
                .set_beacon_data(data)
        }
    };
}