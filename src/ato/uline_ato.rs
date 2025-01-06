
use bveats_rs::{AtsConstantSpeed, AtsHandles, AtsKey, AtsVehicleState, BveAts};
use log::info;
use crate::settings::Settings;
use crate::timer::Timer;

use crate::atc::atc_signal::AtcSignal;


/// ATOの状態を表す
#[derive(Debug, PartialEq)]
enum ATOStatus {
    /// 停止状態
    Stop,
    /// 出発制御
    Departure,
    /// 定速運転制御
    ConstantSpeed,
    /// 力行OFF制御
    PowerOff(AtcSignal),
    /// 減速制御
    Braking(i32, AtcSignal),
    /// 定位置停止制御(90パターン)
    TASC90(i32, f32, f32),
    /// 定位置停止制御(75パターン)
    TASC1(i32, f32, f32),
    /// 定位置停止制御(20パターン)
    TASC2(i32, f32, f32),
    /// 過速防止制御(P3)
    P3(i32, f32, f32),
}
impl Default for ATOStatus {
    fn default() -> Self {
        Self::Stop
    }
}

/// ATOを表す
#[derive(Debug)]
pub struct ULineATO {
    status: ATOStatus,
    before_ato_notch: AtsHandles,
    signal: AtcSignal,
    now_power: i32,
    now_brake: i32,
    before_time: i32,
    before_speed: f32,
    before_acceleration: f32,
    operation_timer: Timer,
    is_not_one_time_braking: bool,

    pub settings: Settings,
}
impl Default for ULineATO {
    fn default() -> Self {
        Self {
            status: Default::default(),
            before_ato_notch: Default::default(),
            signal: Default::default(),
            now_brake: 0,
            now_power: 0,
            before_time: 0,
            before_speed: 0.0,
            before_acceleration: 0.0,
            operation_timer: Timer::new(200),
            is_not_one_time_braking: false,
            settings: Default::default(),
        }
    }
}


impl BveAts for ULineATO {
    fn load(&mut self) {
    }

    fn dispose(&mut self) {
    }

    fn set_vehicle_spec(&mut self, _spec: bveats_rs::AtsVehicleSpec) {
    }

    fn initialize(&mut self, _handle: bveats_rs::AtsInit) {
    }

    fn elapse(&mut self, state: bveats_rs::AtsVehicleState, _panel: &mut [i32], _sound: &mut [i32]) -> bveats_rs::AtsHandles {
        let delta = state.time - self.before_time;
        let acceleration_km_h_s = (state.speed - self.before_speed) / (delta as f32 / 1000.0);

        let atc_brake =  state.speed > self.signal.getSpeed() as f32;
        // ATCブレーキチェック
        if atc_brake {
            self.now_power = 0;
        }

        let result = match self.status {
            ATOStatus::Departure => {
                let target_speed = self.signal.getSpeed() - 5; // ATO目標速度
                if target_speed as f32 <= state.speed {
                    info!("[ATO] Departure->ConstantSpeed");
                    self.status = ATOStatus::ConstantSpeed;
                }

                self.ato_constant_speed(state)
            }
            ATOStatus::ConstantSpeed => {
                let result = self.ato_constant_speed(state);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                result
            }
            ATOStatus::TASC1(_pattern_start_time, beacon_location, target_distance) => {
                if beacon_location.is_nan() {
                    if let ATOStatus::TASC1(_, location, _) = &mut self.status {
                        *location = state.location as f32;
                    }
                }
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                *result
            }
            ATOStatus::TASC2(pattern_start_time, beacon_location, target_distance) => {
                if state.speed < 1.0 {
                    let status = ATOStatus::P3(pattern_start_time, beacon_location, target_distance);
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                if state.speed > self.settings.ato.p2_check_speed {
                    return AtsHandles {
                        brake: self.settings.vehicle.output_brake_notches + 1,
                        power: 0,
                        reverser: 1,
                        constant_speed: AtsConstantSpeed::Disable as i32,
                    }
                }
                if beacon_location.is_nan() {
                    if let ATOStatus::TASC2(_, location, _) = &mut self.status {
                        *location = state.location as f32;
                    }
                }
                // let result = self.ato_tasc(state, pattern_start_time, 20.0);
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                *result
            }
            ATOStatus::TASC90(_pattern_start_time, beacon_location, target_distance) => {
                // let result = self.ato_tasc(state, pattern_start_time, 95.0);
                if beacon_location.is_nan() {
                    if let ATOStatus::TASC90(_, location, _) = &mut self.status {
                        *location = state.location as f32;
                    }
                }
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                *result
            }
            ATOStatus::P3(_pattern_start_time, beacon_location, target_distance) => {
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                
                if state.speed == 0.0 {
                    let status = ATOStatus::Stop;
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }

                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;

                *result
            }
            ATOStatus::Braking(mut time, signal) => {
                if time == -1 {
                    if let ATOStatus::Braking(time, _) = &mut self.status {
                        *time = state.time;
                    }
                    time = state.time;
                    // 一個下現示速度以下
                    let lower_signal_speed = self.signal.getLower().getSpeed();
                    if state.speed < lower_signal_speed as f32 {
                        let status = ATOStatus::ConstantSpeed;
                        info!("[ATO] {:?}→{:?}", self.status, status);
                        self.status = status;
                    }
                }
                // 8秒以上
                if time + self.settings.ato.p4_brake_time < state.time {
                    let status = ATOStatus::ConstantSpeed;
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                // 現示が変位
                if signal != self.signal {
                    let status = ATOStatus::ConstantSpeed;
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }

                if self.operation_timer.is_ready(state.time) {
                    self.now_brake += 1;
                }

                self.now_power = self.now_power.clamp(0, 31);
                self.now_brake = self.now_brake.clamp(0, 23);

                AtsHandles {
                    power: self.now_power,
                    brake: self.now_brake,
                    reverser: 1,
                    constant_speed: AtsConstantSpeed::Disable as i32
                }
            }
            ATOStatus::PowerOff(signal) => {
                // 35km/h以下
                if state.speed < self.settings.ato.p5_lower_limit_speed {
                    let status = ATOStatus::ConstantSpeed;
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                // 上位現示に変化
                if signal.getSpeed() < self.signal.getSpeed() {
                    let status = ATOStatus::ConstantSpeed;
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                // 目標速度-5km/h
                let target_speed = self.signal.getSpeed() - 3;
                if target_speed as f32 - 5.0 < state.speed {
                    let status = ATOStatus::ConstantSpeed;
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }

                AtsHandles {
                    power: 0,
                    brake: 0,
                    reverser: 1,
                    constant_speed: AtsConstantSpeed::Disable as i32
                }
            }
            ATOStatus::Stop => {
                AtsHandles {
                    power: 0,
                    brake: 31,
                    reverser: 0,
                    constant_speed: AtsConstantSpeed::Disable as i32
                }
            }
        };
        self.before_ato_notch = result;
        result
    }

    fn set_power(&mut self, _notch: i32) {
        
    }

    fn set_brake(&mut self, _notch: i32) {
        
    }

    fn set_reverser(&mut self, _notch: i32) {
        
    }

    fn key_down(&mut self, key: bveats_rs::AtsKey) {
        if key == AtsKey::S {
            if self.before_speed != 0.0 {
                return;
            }
            let status = ATOStatus::Departure;
            info!("[ATO] {:?}→{:?}", self.status, status);
            self.status = status;
        }
    }

    fn key_up(&mut self, _key: bveats_rs::AtsKey) {
        
    }

    fn horn_blow(&mut self, _horn_type: bveats_rs::AtsHorn) {
        
    }

    fn door_open(&mut self) {
        
    }

    fn door_close(&mut self) {
        
    }

    fn set_signal(&mut self, signal: i32) {
        self.signal = unsafe { std::mem::transmute(signal as u8) };
    }

    fn set_beacon_data(&mut self, data: bveats_rs::AtsBeaconData) {
        match data.beacon_type {
            1 => { // 第1パターン
                let status = ATOStatus::TASC1(self.before_time, f32::NAN , 350.5);
                info!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            2 => { // 第2パターン
                let status = ATOStatus::TASC2(self.before_time, f32::NAN , 25.5);
                info!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            3 => { // 過速防止
                if let ATOStatus::TASC2(pattern_start_time, beacon_location, target_distance) = self.status {
                    let status = ATOStatus::P3(pattern_start_time, beacon_location, target_distance);
                    info!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                    self.is_not_one_time_braking = false;
                };
            }
            4 => { // 減速制御
                let status = ATOStatus::Braking(-1, self.signal);
                info!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            5 => { // 力行OFF
                if self.before_speed < self.settings.ato.p5_lower_limit_speed {
                    return;
                }
                let status = ATOStatus::PowerOff(self.signal);
                info!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            6 => { // 90パターン
                let status = ATOStatus::TASC90(self.before_time, f32::NAN , 600.5);
                info!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            _ => {}
        }
    }
}

impl ULineATO {
    fn ato_constant_speed(&mut self, state: AtsVehicleState) -> AtsHandles {
        let delta = state.time - self.before_time;
        let acceleration_km_h_s = (state.speed - self.before_speed) / (delta as f32 / 1000.0);

        let speed_2second = state.speed + (acceleration_km_h_s * 1.0);
        let target_speed = self.signal.getSpeed() - 3; // ATO目標速度
        let speed_diff = target_speed as f32 - speed_2second;
        let mut power_notch: i32 = (speed_diff / 0.4) as i32;
        let mut brake_notch = (speed_diff / 0.3) as i32;

        power_notch += ((speed_diff) / 0.75).clamp(-10.0, 10.0) as i32;
        brake_notch += ((speed_diff) / 0.25).clamp(-10.0, 10.0) as i32;

        self.now_power =  power_notch.clamp(0, 31);
        self.now_brake = -brake_notch.clamp(-31, 0);

        AtsHandles {
            power: self.now_power,
            brake: self.now_brake,
            reverser: 1,
            constant_speed: AtsConstantSpeed::Disable as i32
        }
    }
    fn ato_tasc_with_distance(&mut self, state: AtsVehicleState, remaining_distance: f32) -> AtsHandles {
        const MAX_DECELERATION: f32 = 3.50;

        let target_speed = self.ato_tasc_target_speed(remaining_distance);
        
        if target_speed.is_nan() {
            return self.before_ato_notch;
        }
        if target_speed > state.speed + 5.0 {
            return self.ato_constant_speed(state);
        }
        self.is_not_one_time_braking = true;

        let mut output_deceleration = (state.speed.powi(2)/(2.0*remaining_distance)) * (1000.0/3600.0);
        if state.speed > 5.0 {
            output_deceleration = (1.0 / 7.2) * ((state.speed.powi(2) - self.ato_tasc_target_speed(remaining_distance / 2.0).powi(2)) / (remaining_distance / 2.0));
        }
        let mut output_brake = output_deceleration / MAX_DECELERATION * self.settings.vehicle.output_brake_notches as f32; 

        { // 速度超過時のブレーキ補填
            output_brake -= ((target_speed - state.speed) / 1.0).clamp(-10.0, 10.0);
        }

        self.now_power = 0;
        self.now_brake = (output_brake.round() as i32).clamp(0, 31);

        AtsHandles {
            power: self.now_power,
            brake: self.now_brake,
            reverser: 1,
            constant_speed: AtsConstantSpeed::Disable as i32
        }
    }
    
    /// 残距離とTASCパターンから目標速度を求める関数
    fn ato_tasc_target_speed(&self, remaining_distance: f32) -> f32 {
        if true {
            (7.2 * 2.30 * remaining_distance).sqrt()
        } else {
            match self.status {
                ATOStatus::TASC90(_, _, _) => {
                    let deceleration = 2.00;
                    (7.2 * deceleration * (33.625 + remaining_distance)).sqrt()
                }
                ATOStatus::TASC1(_, _, _) => {
                    let deceleration = 2.25;
                    (7.2 * deceleration * (-8.500 + remaining_distance)).sqrt()
                }
                ATOStatus::TASC2(_, _, _) | 
                ATOStatus::P3(_, _, _) => {
                    let deceleration = 1.50;
                    (7.2 * deceleration * remaining_distance).sqrt()
                }
                _ => 0.0
            }
        }
    }
}