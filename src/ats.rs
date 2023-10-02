use crate::api::*;
use crate::voice::*;

const ATS_SOUND_BUZZER: usize = 2;

#[repr(i32)]
#[derive(Debug)]
enum SeishinYamateStation {
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

#[repr(i32)]
#[derive(Debug)]
enum SeishinYamateTrainType {
    None = 0,
    Local = 1,
    OutOfService = 2,
    TestRun = 3,
    Rapid1 = 4,
    Rapid2 = 5,
}

pub struct KobeCitySubwayATS {
    vehicle_spec: AtsVehicleSpec,
    is_changing_signal: bool,
    man_power: i32,
    man_brake: i32,
    man_reverser: i32,
    now_signal: i32,
    start_station: i32,
    destination: i32,
    train_type: i32,
    operation_number: i32,
}

impl KobeCitySubwayATS {
    fn get_signal_speed(&self, signal: i32) -> i32 {
        match signal {
            0 => 0,
            1 => 15,
            2 => 25,
            3 => 45,
            4 => 60,
            5 => 75,
            6 => 90,
            _ => 0,
        }
    }
}

impl BveAts for KobeCitySubwayATS {

    fn load(&mut self) {
        println!("Load");
    }
    fn dispose(&mut self) {
        println!("Dispose");
    }
    fn get_plugin_version(&mut self) -> i32 { 
        println!("GetPluginVersion"); 
        ATS_VERSION 
    }
    fn set_vehicle_spec(&mut self, spec: AtsVehicleSpec) {
        println!("SetVehicleSpec: {:?}", spec);
        self.vehicle_spec = spec;
    }
    fn initialize(&mut self, _handle: AtsInit) {
    }
    fn elapse(&mut self, state: AtsVehicleState, panel: &mut [i32], sound: &mut [i32]) -> AtsHandles {
        // println!("Elapse: {:?}\n{:?}\n{:?}", state, panel, sound);
        if self.is_changing_signal {
            self.is_changing_signal = false;
            sound[ATS_SOUND_BUZZER] = AtsSound::Play as i32;
            println!("ATC信号 変化！");
        } else {
            sound[ATS_SOUND_BUZZER] = AtsSound::Continue as i32;
        }
        if (self.get_signal_speed(self.now_signal) as f32) < state.speed {
            // ATC速度超過
            println!("ATC速度超過！");
            return AtsHandles {
                brake: self.vehicle_spec.brake_notches,
                power: 0,
                reverser: self.man_reverser,
                constant_speed: 0
            }
        }
        AtsHandles {
            brake: self.man_brake,
            power: self.man_power,
            reverser: self.man_reverser,
            constant_speed: 0,
        }
    }
    fn set_power(&mut self, notch: i32) {
        println!("SetPower: {:?}", notch);
        self.man_power = notch;
    }
    fn set_brake(&mut self, notch: i32) {
        println!("SetBrake: {:?}", notch);
        self.man_brake = notch;
    }
    fn set_reverser(&mut self, notch: i32) {
        println!("SetReverser: {:?}", notch);
        self.man_reverser = notch;
    }
    fn key_down(&mut self, key: AtsKey) {
        println!("KeyDown: {:?}", key);
    }
    fn key_up(&mut self, key: AtsKey) {
        println!("KeyUp: {:?}", key);
    }
    fn horn_blow(&mut self, horn_type: AtsHorn) {
        println!("HornBlow: {:?}", horn_type);
    }
    fn door_open(&mut self) {
        println!("DoorOpen");
    }
    fn door_close(&mut self) {
        println!("DoorClose");
    }
    fn set_signal(&mut self, signal: i32) {
        println!("SetSignal: {:?}", signal);
        self.now_signal = signal;
        self.is_changing_signal = true;
    }
    fn set_beacon_data(&mut self, data: AtsBeaconData) {
        println!("SetBeaconData: {:?}", data);
        match data.beacon_type {
            11 => { // 始発駅設定
                if 0 <= data.optional && data.optional <= 17 {
                    self.start_station = data.optional;
                }
            },
            12 => { // 行先設定
                if 0 <= data.optional && data.optional <= 17 {
                    self.destination = data.optional;
                }
            },
            13 => { // 種別設定
                if 0 <= data.optional && data.optional <= 5 {
                    self.train_type = data.optional;
                }
            },
            14 => { // 運番設定
                if 0 <= data.optional && data.optional <= 99 {
                    self.train_type = data.optional;
                }
            },
            _ => println!("[ATS_WARN]: 定義されていない地上子番号です。")
        }
    }
}

impl Default for KobeCitySubwayATS {
    fn default() -> Self {
        Self { 
            man_power: 0, 
            man_brake: 0, 
            man_reverser: 0, 
            now_signal: 0, 
            vehicle_spec: AtsVehicleSpec::default(), 
            is_changing_signal: false,
            start_station: 0,
            destination: 0,
            train_type: 0,
            operation_number: 0,
        }
    }
}