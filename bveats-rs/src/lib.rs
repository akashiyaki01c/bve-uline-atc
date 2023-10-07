#![allow(unused)]

pub const ATS_VERSION: i32 = 0x00020000;

#[repr(i32)]
#[derive(Debug)]
pub enum AtsKey {
    S = 0,
    A1 = 1,
    A2 = 2,
    B1 = 3,
    B2 = 4,
    C1 = 5,
    C2 = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15
}

#[repr(i32)]
#[derive(Debug)]
pub enum AtsInit {
    Removed = 2,
    Emg = 1,
    Svc = 0,
}

#[repr(i32)]
#[derive(Debug)]
pub enum AtsSound {
    Stop = -10000,
    Play = 1,
    PlayLooping = 0,
    Continue = 2,
}

#[repr(i32)]
#[derive(Debug)]
pub enum AtsHorn {
    Primary = 0,
    Secondary = 1,
    Music = 2,
}

#[repr(i32)]
#[derive(Debug)]
pub enum AtsConstantSpeed {
    Continue = 0,
    Enable = 1,
    Disable = 2,
}

#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsVehicleSpec {
    pub brake_notches: i32,
    pub power_notches: i32,
    pub ats_notch: i32,
    pub b67_notch: i32,
    pub cars: i32,
}

#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsVehicleState {
    pub location: f64,
    pub speed: f32,
    pub time: i32,
    pub bc_pressure: f32,
    pub mr_pressure: f32,
    pub er_pressure: f32,
    pub bp_pressure: f32,
    pub sap_pressure: f32,
    pub current: i32,
}

#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsBeaconData {
    pub beacon_type: i32,
    pub signal: i32,
    pub distance: f32,
    pub optional: i32,
}

#[repr(C)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct AtsHandles {
    pub brake: i32,
    pub power: i32,
    pub reverser: i32,
    pub constant_speed: i32,
}

pub trait BveAts {
    fn load(&mut self);
    fn dispose(&mut self);
    fn get_plugin_version(&mut self) -> i32 { ATS_VERSION }
    fn set_vehicle_spec(&mut self, spec: AtsVehicleSpec);
    fn initialize(&mut self, handle: AtsInit);
    fn elapse(&mut self, state: AtsVehicleState, panel: &mut [i32], sound: &mut [i32]) -> AtsHandles;
    fn set_power(&mut self, notch: i32);
    fn set_brake(&mut self, notch: i32);
    fn set_reverser(&mut self, notch: i32);
    fn key_down(&mut self, key: AtsKey);
    fn key_up(&mut self, key: AtsKey);
    fn horn_blow(&mut self, horn_type: AtsHorn);
    fn door_open(&mut self);
    fn door_close(&mut self);
    fn set_signal(&mut self, signal: i32);
    fn set_beacon_data(&mut self, data: AtsBeaconData);
}

#[macro_export]
macro_rules! ats_main {
    ($t: ty) => {
        use ::std::sync::Mutex;
        use ::std::sync::OnceLock;
        use ::bveats_rs::*;
        static ATS: OnceLock<Mutex<ats::KobeCitySubwayATS>> = OnceLock::new();

        #[no_mangle]
        pub unsafe extern "C" fn Load() {
            ATS.get_or_init(|| Mutex::new(<$t>::default())).lock().unwrap().load();
        }

        #[no_mangle]
        pub unsafe extern "C" fn Dispose() {
            ATS.get().unwrap().lock().unwrap().dispose()
        }

        #[no_mangle]
        pub unsafe extern "C" fn GetPluginVersion() -> i32 {
            ATS.get().unwrap().lock().unwrap().get_plugin_version()
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetVehicleSpec(spec: AtsVehicleSpec) {
            ATS.get().unwrap().lock().unwrap().set_vehicle_spec(spec)
        }

        #[no_mangle]
        pub unsafe extern "C" fn Initialize(brake: i32) {
            println!("{}", brake);
            ATS.get().unwrap().lock().unwrap().initialize(std::mem::transmute(brake))
        }

        #[no_mangle]
        pub unsafe extern "C" fn Elapse(state: AtsVehicleState, panel: *mut i32, sound: *mut i32) -> AtsHandles {
            ATS.get().unwrap().lock().unwrap().elapse(state, 
                std::slice::from_raw_parts_mut(panel, 256), 
                std::slice::from_raw_parts_mut(sound, 256))
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetPower(notch: i32) {
            ATS.get().unwrap().lock().unwrap().set_power(notch)
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetBrake(notch: i32) {
            ATS.get().unwrap().lock().unwrap().set_brake(notch)
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetReverser(notch: i32) {
            ATS.get().unwrap().lock().unwrap().set_reverser(notch)
        }

        #[no_mangle]
        pub unsafe extern "C" fn KeyDown(key: i32) {
            ATS.get().unwrap().lock().unwrap().key_down(std::mem::transmute(key))
        }

        #[no_mangle]
        pub unsafe extern "C" fn KeyUp(key: i32) {
            ATS.get().unwrap().lock().unwrap().key_up(std::mem::transmute(key))
        }

        #[no_mangle]
        pub unsafe extern "C" fn HornBlow(horn: i32) {
            ATS.get().unwrap().lock().unwrap().key_up(std::mem::transmute(horn))
        }

        #[no_mangle]
        pub unsafe extern "C" fn DoorOpen() {
            ATS.get().unwrap().lock().unwrap().door_open()
        }

        #[no_mangle]
        pub unsafe extern "C" fn DoorClose() {
            ATS.get().unwrap().lock().unwrap().door_close()
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetSignal(signal: i32) {
            ATS.get().unwrap().lock().unwrap().set_signal(signal)
        }

        #[no_mangle]
        pub unsafe extern "C" fn SetBeaconData(data: AtsBeaconData) {
            ATS.get().unwrap().lock().unwrap().set_beacon_data(data)
        }
    };
}